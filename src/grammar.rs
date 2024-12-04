use std::collections::{HashMap, HashSet};
use crate::trie::Trie;

const DEBUG: bool = true;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Grammar {
    non_terminals: HashSet<String>,
    terminals: HashSet<String>,
    start_symbol: Option<String>,
    pub(crate) productions: HashMap<String, Vec<String>>,
    pub first_sets: HashMap<String, HashSet<String>>,
    pub follow_sets: HashMap<String, HashSet<String>>,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar {
            non_terminals: HashSet::new(),
            terminals: HashSet::new(),
            start_symbol: None,
            productions: HashMap::new(),
            first_sets: HashMap::new(),
            follow_sets: HashMap::new(),
        }
    }

    pub fn add_production(&mut self, non_terminal: &str, production: Vec<&str>, is_start_symbol: bool) {
        self.non_terminals.insert(non_terminal.to_string());
        let entry: &mut Vec<String> = self.productions.entry(non_terminal.to_string()).or_insert_with(Vec::new);
        for p in production {
            // 将产生式添加到对应的列表中
            entry.push(p.to_string());
            // 按字符迭代处理每个符号

            self.terminals.insert("$".to_string());

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

        // 设置开始符号
        if is_start_symbol {
            self.start_symbol = Some(non_terminal.to_string());
        }
        /*
        if !self.start_symbol.is_none() {
            self.eliminate_left_recursion();
            self.first_sets = self.calculate_first_sets();
            self.calculate_follow_sets();
        }
        */
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

    pub fn calculate_follow_sets(&mut self) -> HashMap<String, HashSet<String>> {
        // 初始化 FOLLOW 集
        for nt in &self.non_terminals {
            self.follow_sets.entry(nt.clone()).or_insert_with(HashSet::new);
        }
        // 设置起始符号的 FOLLOW 集，通常包含一个特殊的结束符号 "$"
        if let Some(start_symbol) = self.start_symbol.clone() {
            self.follow_sets.entry(start_symbol.clone()).or_insert_with(HashSet::new).insert("$".to_string());
        }

        loop {
            let mut changed = false;
            for (lhs, productions) in &self.productions {
                for production in productions {
                    let symbols: Vec<String> = production.chars().map(|c| c.to_string()).collect();
                    let mut trailer = self.follow_sets.get(lhs).unwrap().clone();

                    for i in (0..symbols.len()).rev() {
                        let symbol = &symbols[i];
                        if self.non_terminals.contains(symbol) {
                            let before_size = self.follow_sets[symbol].len();
                            self.follow_sets.get_mut(symbol).unwrap().extend(trailer.clone());
                            if self.follow_sets[symbol].len() != before_size {
                                changed = true;
                            }
                        }

                        if i > 0 {
                            trailer.clear();
                            if self.non_terminals.contains(&symbols[i - 1]) {
                                if let Some(first_next) = self.first_sets.get(symbol) {
                                    trailer.extend(first_next.iter().filter(|&n| n != "ε").cloned());
                                    if first_next.contains("ε") {
                                        trailer.extend(self.follow_sets.get(lhs).unwrap().iter().cloned());
                                    }
                                }
                            }
                        }
                        else {
                            trailer.clear();
                            trailer.extend(self.follow_sets.get(lhs).unwrap().iter().cloned());
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }

        self.follow_sets.clone()
    }

    // Check if the grammar is LL(1)
    pub fn is_ll1(&self) -> bool {
        for (non_terminal, productions) in &self.productions {
            for i in 0..productions.len() {
                for j in (i + 1)..productions.len() {
                    let first_i = self.first_of(&productions[i]);
                    let first_j = self.first_of(&productions[j]);

                    // Check FIRST sets intersection
                    if !first_i.is_disjoint(&first_j) {
                        return false;
                    }

                    // If one production can derive epsilon, check FOLLOW sets
                    if first_i.contains("ε") {
                        let follow = self.follow_sets.get(non_terminal).unwrap();
                        if !follow.is_disjoint(&first_j) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    // Helper function to compute the FIRST set of a given production
    pub fn first_of(&self, production: &str) -> HashSet<String> {
        let mut first_set = HashSet::new();

        for symbol in production.chars() {
            let symbol_str = symbol.to_string();
            if self.terminals.contains(&symbol_str) {
                first_set.insert(symbol_str);
                break;
            } else if let Some(non_terminal_first) = self.first_sets.get(&symbol_str) {
                for item in non_terminal_first {
                    if item != "ε" {
                        first_set.insert(item.clone());
                    }
                }

                if !non_terminal_first.contains("ε") {
                    break;
                }
            }
        }

        if first_set.is_empty() {
            first_set.insert("ε".to_string());
        }

        first_set
    }

    // Generate predictive parsing table
    pub fn create_predictive_parsing_table(&self) -> HashMap<(String, String), Vec<String>> {
        let mut table: HashMap<(String, String), Vec<String>> = HashMap::new();

        for (non_terminal, productions) in &self.productions {
            for production in productions {
                let first_set = self.first_of(production);
                for terminal in &first_set {
                    if terminal != "ε" {
                        table.insert((non_terminal.clone(), terminal.clone()), production.split_whitespace().map(String::from).collect());
                    }
                }
                if first_set.contains("ε") {
                    if let Some(follow_set) = self.follow_sets.get(non_terminal) {
                        for terminal in follow_set {
                            table.insert((non_terminal.clone(), terminal.clone()), production.split_whitespace().map(String::from).collect());
                        }
                    }
                }
            }
        }

        table
    }

    // LL(1) parser implementation using predictive parsing table
    pub fn ll1_parse(&self, input: &str) -> Result<(), String> {
        let table = self.create_predictive_parsing_table();
        let mut stack = vec!["$".to_string()];
        if let Some(start) = &self.start_symbol {
            stack.push(start.clone());
        } else {
            return Err("Start symbol is not defined".to_string());
        }

        let mut input_buffer: Vec<String> = input.chars().map(|c| c.to_string()).collect::<Vec<_>>();
        input_buffer.push("$".to_string());

        while let Some(top) = stack.pop() {
            let current_symbol = input_buffer.first().unwrap().clone();
            if current_symbol != "$".to_string() {
                if self.terminals.contains(&top) || top == "$" {
                    if top == current_symbol {
                        input_buffer.remove(0);
                        display_stack(&stack);
                    } else {
                        return Err(format!("Syntax error: expected {}, found {}", top, current_symbol));
                    }
                }
                else if self.non_terminals.contains(&top) {
                    if let Some(productions) = table.get(&(top.to_string(), current_symbol.clone())) {
                        for production in productions {
                            for symbol in production.chars().rev() {
                                if symbol != 'ε' {
                                    stack.push(symbol.to_string());
                                }
                            }
                            display_stack(&stack);
                        }
                    } else {
                        return Err(format!("Syntax error: no rule for {} when seeing {}", top, current_symbol));
                    }
                } else {
                    return Err(format!("Unknown symbol on stack: {}", top));
                }
            }

        }

        if input_buffer.len() == 1 && input_buffer[0] == "$" {
            Ok(())
        } else {
            Err("Syntax error: input not fully consumed".to_string())
        }
    }

    pub fn display(&self) {
        for (non_terminal, productions) in &self.productions {
            println!("{} -> {}", non_terminal, productions.join(" | "));
        }
    }

    pub fn display_predictive_parsing_table(&self, table: &HashMap<(String, String), Vec<String>>) {
        // Collect all non-terminals and terminals for table formatting
        let mut non_terminals: HashSet<String> = HashSet::new();
        let mut terminals: HashSet<String> = HashSet::new();

        for ((non_terminal, terminal), _) in table {
            non_terminals.insert(non_terminal.clone());
            terminals.insert(terminal.clone());
        }

        let mut non_terminals: Vec<String> = non_terminals.into_iter().collect();
        let mut terminals: Vec<String> = terminals.into_iter().collect();
        non_terminals.sort();
        terminals.sort();

        // Print header row
        print!("\nPredictive Parsing Table:\n\n");
        print!("{:>10} |", "");
        for terminal in &terminals {
            print!("{:>15} |", terminal);
        }
        println!();
        print!("{:->15}-", "");
        for _ in &terminals {
            print!("{:->15}-", "");
        }
        println!();

        for non_terminal in &non_terminals {
            if non_terminal == self.start_symbol.as_ref().unwrap() {
                print!("{:>10} |", non_terminal);
                for terminal in &terminals {
                    if let Some(production) = table.get(&(non_terminal.clone(), terminal.clone())) {
                        // 修改这里来包含非终结符和产生式的格式
                        let production_str = format!("{} -> {}", non_terminal, production.join(" "));
                        print!("{:>15} |", production_str);
                    }
                    else {
                        print!("{:>15} |", "");
                    }
                }
                println!();
            }
        }
        for non_terminal in &non_terminals {
            if non_terminal != self.start_symbol.as_ref().unwrap() {
                print!("{:>10} |", non_terminal);
                for terminal in &terminals {
                    if let Some(production) = table.get(&(non_terminal.clone(), terminal.clone())) {
                        // 修改这里来包含非终结符和产生式的格式
                        let production_str = format!("{} -> {}", non_terminal, production.join(" "));
                        print!("{:>15} |", production_str);
                    }
                    else {
                        print!("{:>15} |", "");
                    }
                }
                println!();
            }
        }
    }

}

// Function to display the current stack status
fn display_stack(stack: &Vec<String>) {
    if DEBUG {
        let stack_content: Vec<String> = stack.iter().map(|s| s.to_string()).collect();
        println!("Stack: [{}]", stack_content.join(", "));
    }
}