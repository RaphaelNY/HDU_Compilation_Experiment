use std::collections::HashMap;
use std::io;
use std::io::Write;

pub mod grammar;
pub mod trie;

pub fn input() -> (Vec<String>, HashMap<String ,Vec<String>>) {
	println!("Enter your productions (e.g., S->AB or A->aA|ε). Press Enter $ with no input to finish:");
	let mut productions: HashMap<String ,Vec<String>> = HashMap::new();
	let mut non_terminals = Vec::new();
	loop {
		println!("Enter production:");
		io::stdout().flush().unwrap();

		// 读取输入
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		let input = input.trim();
		if input == "$" {
			break;
		}
		let mut parts = input.split("->");
		let non_terminal = parts.next().unwrap().trim().to_string();
		non_terminals.push(non_terminal.clone());
		let productions_str = parts.next().unwrap().trim();
		let production = productions_str.split("|").map(|s| s.trim().to_string()).collect();
		productions.insert(non_terminal, production);
	}
	(non_terminals, productions)
}

pub fn add_prod(grammar: &mut grammar::Grammar, non_terminals: Vec<String>, productions: HashMap<String ,Vec<String>>) {
	let mut is_start = true;
	for non_terminal in non_terminals {
		let production = productions.get(&non_terminal).unwrap();
		let prod: Vec<_> = production.iter().map(|c|c.as_str()).collect();
		grammar.add_production(&non_terminal, prod, is_start);
		is_start = false;
	}
}

pub fn task3_1(grammar: &mut grammar::Grammar) {
	println!("Original Grammar:");
	grammar.display();
	grammar.eliminate_left_recursion();
	println!("\nGrammar after eliminating left recursion:");
	grammar.display();
}

pub fn task3_2(grammar: &mut grammar::Grammar) {
	println!("Original Grammar:");
	grammar.display();
	grammar.eliminate_left_common_factor();
	println!("\nGrammar after eliminating left common factor:");
	grammar.display();
}

pub fn task3_3(grammar: &mut grammar::Grammar) {
	println!("Original Grammar:");
	grammar.display();
	grammar.first_sets = grammar.calculate_first_sets();
	grammar.calculate_follow_sets();
	let first_sets = grammar.first_sets.clone();
	println!("\nFIRST Sets:");
	for (nt, fs) in first_sets {
		println!("FIRST({}) = {:?}", nt, fs);
	}
	let follow_sets = grammar.follow_sets.clone();
	println!("\nFOLLOW Sets:");
	for (nt, fs) in follow_sets {
		println!("FOLLOW({}) = {:?}", nt, fs);
	}
}

pub fn task3_4(grammar: &mut grammar::Grammar) {
	println!("Original Grammar:");
	grammar.display();
	grammar.first_sets = grammar.calculate_first_sets();
	grammar.calculate_follow_sets();

	if grammar.is_ll1() { println!("The grammar is LL(1)");	} else { println!("The grammar is not LL(1)"); }
	let table = grammar.create_predictive_parsing_table();
	grammar.display_predictive_parsing_table(&table);

	match grammar.ll1_parse("aab") {
		Ok(_) => println!("Parsing successful"),
		Err(err) => println!("Parsing failed: {}", err),
	}
}


#[cfg(test)]
mod tests {
	// use super::*;

	#[test]
	fn test_grammar() {

	}
}