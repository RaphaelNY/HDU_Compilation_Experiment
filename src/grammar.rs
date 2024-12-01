use std::collections::{HashMap, HashSet};
use crate::trie::{Trie};

const DEBUG: bool = true;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Grammar {
    non_terminals: HashSet<String>,
    terminals: HashSet<String>,
    pub(crate) productions: HashMap<String, Vec<String>>,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar {
            non_terminals: HashSet::new(),
            terminals: HashSet::new(),
            productions: HashMap::new(),
        }
    }

    pub fn add_production(&mut self, non_terminal: &str, production: Vec<&str>) {
        self.non_terminals.insert(non_terminal.to_string());
        let entry: &mut Vec<String> = self.productions.entry(non_terminal.to_string()).or_insert_with(Vec::new);
        for p in production {
            // 将产生式添加到对应的列表中
            entry.push(p.to_string());
            // 按字符迭代处理每个符号
            for ch in p.chars() {
                let symbol = ch.to_string();
                // 检查是否为非终结符
                if !self.non_terminals.contains(&symbol) && symbol != " " {
                    self.terminals.insert(symbol);
                }
            }
        }
        for non_terminal in self.non_terminals.iter() {
            if self.terminals.contains(non_terminal) {
                self.terminals.remove(non_terminal);
            }
        }
    }

    pub fn eliminate_left_recursion(&mut self) {
        let non_terminals: Vec<_> = self.non_terminals.iter().cloned().collect();
        for i in 0..non_terminals.len() {
            let ai = &non_terminals[i];
            for j in 0..i {
                let aj = &non_terminals[j];
                if let Some(ai_productions) = self.productions.get(ai) {
                    let mut new_productions = Vec::new();
                    for prod in ai_productions.iter() {
                        if prod.starts_with(aj) {
                            if let Some(aj_productions) = self.productions.get(aj) {
                                for aj_prod in aj_productions.iter() {
                                    let new_prod = aj_prod.clone() + &prod[aj.len()..];
                                    new_productions.push(new_prod);
                                }
                            }
                        }
                        else {
                            new_productions.push(prod.clone());
                        }
                    }
                    if let Some(ai_productions_mut) = self.productions.get_mut(ai) {
                        *ai_productions_mut = new_productions;
                    }
                }
            }
            self.eliminate_direct_left_recursion(ai);
        }
    }

    fn eliminate_direct_left_recursion(&mut self, non_terminal: &str) {
        if let Some(productions) = self.productions.get(non_terminal) {
            let mut direct_recursive = Vec::new();
            let mut non_recursive = Vec::new();

            for prod in productions {
                if prod.starts_with(non_terminal) {
                    direct_recursive.push(prod[non_terminal.len()..].to_string());
                } else {
                    non_recursive.push(prod.clone());
                }
            }

            if !direct_recursive.is_empty() {
                let new_non_terminal = format!("{}'", non_terminal);
                self.non_terminals.insert(new_non_terminal.clone());

                let new_productions = non_recursive
                    .into_iter()
                    .map(|prod| format!("{}{}", prod, new_non_terminal))
                    .collect::<Vec<_>>();

                let mut new_recursive_productions = direct_recursive
                    .into_iter()
                    .map(|prod| format!("{}{}", prod, new_non_terminal))
                    .collect::<Vec<_>>();
                new_recursive_productions.push("ε".to_string());

                self.productions
                    .insert(new_non_terminal.clone(), new_recursive_productions);
                self.productions.insert(non_terminal.to_string(), new_productions);
            }
        }
    }

    // 添加一个方法来构建并返回一个非终结符的Trie树
    pub fn build_trie_for_nonterminal(&self, non_terminal: &str) -> Option<Trie> {
        if let Some(productions) = self.productions.get(non_terminal) {
            let mut trie = Trie::new();
            for production in productions {
                trie.insert(production);
            }
            if DEBUG { println!("{}", trie) }
            Some(trie)
        } else {
            None
        }
    }

    pub fn eliminate_left_common_factor(&mut self) {
        let mut new_rules: HashMap<String, Vec<String>> = HashMap::new();
        for non_terminal in self.non_terminals.clone().iter() {
            if let Some(productions) = self.productions.clone().get(non_terminal) {
                if let Some(trie) = self.build_trie_for_nonterminal(non_terminal) {
                    let lcp: Vec<(String, Vec<String>)> = trie.extract_longest_common_prefix();
                    let prefixs: Vec<_> = lcp.iter().map(|(prefix, _branches)| prefix).collect();
                    let mut new_productions: Vec<String> = vec![];
                    let mut new_non_terminal = non_terminal.clone();
                    // update grammar
                    for prefix in prefixs.clone() {
                        new_non_terminal = format!("{}'", new_non_terminal);
                        let mut suffixes = Vec::new();

                        for production in productions {
                            if let Some(suffix) = production.strip_prefix(prefix) {
                                suffixes.push(suffix.to_string());
                            }
                        }
                        // 如果找到了匹配的前缀产生式
                        if !suffixes.is_empty() {
                            self.non_terminals.insert(new_non_terminal.clone());
                            new_rules.insert(new_non_terminal.clone(), suffixes.iter().map(|s| if s.is_empty() { "ε".into() } else { s.clone() }).collect());
                            new_productions.push(format!("{}{}", prefix, new_non_terminal));
                        }
                    }

                    'outer: for production in productions {
                        for prefix in prefixs.clone() {
                            if production.starts_with(prefix) {
                                continue 'outer;  // 如果匹配到任何一个前缀，跳过当前产生式，继续检查下一个
                            }
                        }
                        // 如果没有匹配到任何前缀，添加到结果列表
                        new_productions.push(production.clone());
                    }

                    // 更新当前非终结符的产生式
                    self.productions.insert(non_terminal.to_string(), new_productions.clone());
                }
            }
        }
        // 合并新的非终结符和产生式到文法中
        self.productions.extend(new_rules);
    }

    pub fn eliminate_first(&self, symbol: &str) -> HashSet<String> {
        let mut first_set = HashSet::new();
        if self.terminals.contains(symbol) {
            first_set.insert(symbol.to_string());
            return first_set;
        }
        if let Some(productions) = self.productions.get(symbol) {
            'productions: for prod in productions {
                let symbols: Vec<String> = prod.chars().map(|c| c.to_string()).collect();
                let mut all_nullable = true;

                for sym in symbols {
                    if sym == "ε" {
                        first_set.insert("ε".to_string());
                        continue;
                    } else if self.terminals.contains(&sym) {
                        first_set.insert(sym.to_string());
                        continue 'productions; // Stop after adding first terminal
                    }

                    let nested_first = self.eliminate_first(&sym);
                    first_set.extend(nested_first.clone().into_iter().filter(|s| s != "ε")); // Add all except ε

                    if !nested_first.contains("ε") {
                        all_nullable = false;
                        break; // Stop if current symbol cannot produce ε
                    }
                }

                if all_nullable {
                    first_set.insert("ε".to_string()); // If all symbols can produce ε, add ε
                }
            }
        }
        first_set
    }

    pub fn calculate_first_sets(&self) -> HashMap<String, HashSet<String>> {
        let mut first_sets = HashMap::new();
        for nt in &self.non_terminals {
            first_sets.insert(nt.clone(), self.eliminate_first(nt));
        }
        first_sets
    }

    pub fn display(&self) {
        for (non_terminal, productions) in &self.productions {
            println!("{} -> {}", non_terminal, productions.join(" | "));
        }
    }
}