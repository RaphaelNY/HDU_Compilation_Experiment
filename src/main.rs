use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Grammar {
    non_terminals: HashSet<String>,
    terminals: HashSet<String>,
    productions: HashMap<String, Vec<String>>,
}

impl Grammar {
    fn new() -> Self {
        Grammar {
            non_terminals: HashSet::new(),
            terminals: HashSet::new(),
            productions: HashMap::new(),
        }
    }

    fn add_production(&mut self, non_terminal: &str, production: Vec<&str>) {
        self.non_terminals.insert(non_terminal.to_string());
        let entry = self.productions.entry(non_terminal.to_string()).or_insert_with(Vec::new);
        for p in production {
            entry.push(p.to_string());
        }
    }

    fn eliminate_left_recursion(&mut self) {
        let non_terminals: Vec<_> = self.non_terminals.iter().cloned().collect();
        for i in 0..non_terminals.len() {
            let ai = &non_terminals[i];
            for j in 0..i {
                let aj = &non_terminals[j];
                if let Some(ai_productions) = self.productions.get_mut(ai) {
                    let mut new_productions = Vec::new();
                    for prod in ai_productions.iter() {
                        if prod.starts_with(aj) {
                            if let Some(aj_productions) = self.productions.get(aj) {
                                for aj_prod in aj_productions {
                                    let new_prod = aj_prod.clone() + &prod[aj.len()..];
                                    new_productions.push(new_prod);
                                }
                            }
                        } else {
                            new_productions.push(prod.clone());
                        }
                    }
                    *ai_productions = new_productions;
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

    fn display(&self) {
        for (non_terminal, productions) in &self.productions {
            println!("{} -> {}", non_terminal, productions.join(" | "));
        }
    }
}

fn main() {
    let mut grammar = Grammar::new();
    grammar.add_production("S", vec!["S+T", "T"]);
    grammar.add_production("T", vec!["T*F", "F"]);
    grammar.add_production("F", vec!["(E)", "id"]);

    println!("Original Grammar:");
    grammar.display();

    grammar.eliminate_left_recursion();

    println!("\nGrammar after eliminating left recursion:");
    grammar.display();
}
