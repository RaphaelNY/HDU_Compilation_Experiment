use task3::grammar::Grammar;
fn main() {
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["apple", "apply", "application", "ball", "bat", "bath", "Xb"]);
    grammar.add_production("X", vec!["ab", "ac", "ad"]);

    println!("Original Grammar:");
    grammar.display();
    println!();

    grammar.eliminate_left_common_factor();
    println!("\nGrammar after eliminating left common factor:");
    grammar.display();
}
/*
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["apple", "apply", "application", "ball", "bat", "bath", "Xb"]);
    grammar.add_production("X", vec!["ab", "ac", "ad"]);

    println!("Original Grammar:");
    grammar.display();

    grammar.eliminate_left_recursion();
    println!("no left recursion Grammar:");
    grammar.display();

    grammar.eliminate_left_common_factor();

    println!("\nGrammar after eliminating left common factor:");
    grammar.display();
*/

/*
    let mut grammar = Grammar::new();
    grammar.add_production("S", vec!["S+T", "T"]);
    grammar.add_production("T", vec!["T*F", "F"]);
    grammar.add_production("F", vec!["(E)", "id"]);

    println!("Original Grammar:");
    grammar.display();

    grammar.eliminate_left_recursion();

    println!("\nGrammar after eliminating left recursion:");
    grammar.display();
*/