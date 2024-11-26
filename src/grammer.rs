use std::collections::{HashMap, HashSet};
use trie::TrieNode;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Grammar {
    non_terminals: HashSet<String>,
    terminals: HashSet<String>,
    productions: HashMap<String, Vec<String>>,
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
        // entry: exist or not; 
        let entry: &mut Vec<String> = self.productions.entry(non_terminal.to_string()).or_insert_with(Vec::new);
        for p in production {
            entry.push(p.to_string());
        }
    }

    pub fn eliminate_left_recursion(&mut self) {
        let non_terminals: Vec<_> = self.non_terminals.iter().cloned().collect();
        for i in 0..non_terminals.len() {
            let ai = &non_terminals[i];
            for j in 0..i {
                let aj = &non_terminals[j];
                // 替换 A_i 中以 A_j 开头的候选式
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
        // 关系式中存在non_terminal的生产式 
        if let Some(productions) = self.productions.get(non_terminal) {
            let mut direct_recursive = Vec::new();
            let mut non_recursive = Vec::new();

            for prod in productions {
                if prod.starts_with(non_terminal) {
                    // 从prod中消去开头的非终结符，存入direct_recursive（包含直接左递归的产生式）
                    direct_recursive.push(prod[non_terminal.len()..].to_string());
                } else {
                    non_recursive.push(prod.clone());
                }
            }

            if !direct_recursive.is_empty() { // 存在直接左递归
                // 生成新的非终结符
                let new_non_terminal = format!("{}'", non_terminal);
                self.non_terminals.insert(new_non_terminal.clone());
                let mut new_productions = Vec::new();

                for prod in &non_recursive {
                    new_productions.push(format!("{}{}", prod, new_non_terminal));
                }

                let mut new_recursive_productions = Vec::new();
                for prod in &direct_recursive {
                    new_recursive_productions.push(format!("{}{}", prod, new_non_terminal));
                }
                new_recursive_productions.push("ε".to_string());

                self.productions.insert(new_non_terminal.clone(), new_recursive_productions);
                self.productions.insert(non_terminal.to_string(), new_productions);
            }
        }
    }

    /// 左公共因子提取
    fn eliminate_left_common_factor(&mut self) {
        let mut new_productions = HashMap::new();
        let mut new_non_terminals = Vec::new();

        for (non_terminal, candidates) in &self.productions {
            let mut trie = TrieNode::default();
            for candidate in candidates {
                trie.insert(candidate);
            }

            let extracted = trie.extract_common_prefix(non_terminal);
            for (nt, prod) in extracted.0 {
                new_productions.insert(nt, prod);
            }
            for nt in extracted.1 {
                new_non_terminals.push(nt);
            }
        }

        // 更新文法
        self.productions.extend(new_productions);
        self.non_terminals.extend(new_non_terminals);
    }

    pub fn display(&self) {
        for (non_terminal, productions) in &self.productions {
            println!("{} -> {}", non_terminal, productions.join(" | "));
        }
    }
}
