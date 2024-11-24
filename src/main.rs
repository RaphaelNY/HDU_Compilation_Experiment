use task3::Grammar;
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
