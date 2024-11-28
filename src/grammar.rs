use std::collections::{HashMap, HashSet};
use crate::trie::TrieNode;

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

    /// 消除左公共因子
    pub fn eliminate_left_common_factor(&mut self) {
        let mut new_productions = HashMap::new(); // 用于存储新非终结符的规则
        let mut new_non_terminals = HashSet::new(); // 用于记录新生成的非终结符

        for (non_terminal, candidates) in &self.productions {
            // 分组候选式
            let groups = group_candidates(candidates);
            let mut updated_parent_rules = HashSet::new(); // 用于去重的父规则

            for (prefix, group_candidates) in groups {
                let mut trie = TrieNode::default();
                for candidate in group_candidates {
                    trie.insert(&candidate);
                }

                // 打印生成的 Trie 树
                println!("\nTrie for group {} in {}:", prefix, non_terminal);
                trie.display(0, "".to_string());

                // 提取公共因子
                let extracted = trie.extract_common_prefix(non_terminal);

                // 提取父规则部分
                if let Some(common_prefix_rules) = extracted.0.get(non_terminal) {
                    for rule in common_prefix_rules {
                        updated_parent_rules.insert(rule.clone());
                    }
                }

                // 合并新生成的规则到 new_productions
                for (key, value) in extracted.0 {
                    let unique_rules: HashSet<String> = value.into_iter().collect();
                    new_productions
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .extend(unique_rules.into_iter());
                }

                // 添加新非终结符到集合
                new_non_terminals.extend(extracted.1.into_iter());
            }
        }

        // 更新新生成的非终结符规则
        for (key, value) in new_productions {
            let unique_rules: HashSet<String> = value.into_iter().collect(); // 再次去重
            self.productions.insert(key, unique_rules.into_iter().collect());
        }

        // 更新非终结符集合
        self.non_terminals.extend(new_non_terminals);
    }

    pub fn display(&self) {
        for (non_terminal, productions) in &self.productions {
            println!("{} -> {}", non_terminal, productions.join(" | "));
        }
    }
}


// 将候选式按照前缀分组
fn group_candidates(candidates: &[String]) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for candidate in candidates {
        // 按第一个字符分组
        let prefix = candidate.chars().next().unwrap().to_string();
        groups.entry(prefix).or_insert_with(Vec::new).push(candidate.clone());
    }

    groups
}