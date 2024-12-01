use task3::grammar::Grammar;
fn main() {
    let mut grammar = Grammar::new();

    grammar.add_production("Q", vec!["R Q'"]);
    grammar.add_production("Q'", vec!["+ R Q'", "ε"]);
    grammar.add_production("R", vec!["c R'"]);
    grammar.add_production("R'", vec!["* c R'", "ε"]);

    println!("Original Grammar:");
    grammar.display();

    let first_sets = grammar.calculate_first_sets();
    println!("\nFIRST Sets:");
    for (nt, fs) in first_sets {
        println!("FIRST({}) = {:?}", nt, fs);
    }
}

/* 3.1
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

/* 3.2
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["apple", "apply", "application", "ball", "bat", "bath", "Xb"]);
    grammar.add_production("X", vec!["ab", "ac", "ad"]);

    println!("Original Grammar:");
    grammar.display();

    grammar.eliminate_left_common_factor();
    println!("\nGrammar after eliminating left common factor:");
    grammar.display();
*/

/* 3.3
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["AB"]);
    grammar.add_production("A", vec!["a", "ε"]);
    grammar.add_production("B", vec!["b"]);

    grammar.add_production("E", vec!["T X"]);
    grammar.add_production("X", vec!["+ T X", "ε"]);
    grammar.add_production("T", vec!["F Y"]);
    grammar.add_production("Y", vec!["* F Y", "ε"]);
    grammar.add_production("F", vec!["( E )", "d"]);

    grammar.add_production("P", vec!["i E t S", "a"]);
    grammar.add_production("E", vec!["b"]);
    grammar.add_production("T", vec![";"]);
    grammar.add_production("S", vec!["a S e", "ε"]);

    grammar.add_production("Q", vec!["R Q'"]);
    grammar.add_production("Q'", vec!["+ R Q'", "ε"]);
    grammar.add_production("R", vec!["c R'"]);
    grammar.add_production("R'", vec!["* c R'", "ε"]);


    println!("Original Grammar:");
    grammar.display();

    let first_sets = grammar.calculate_first_sets();
    println!("\nFIRST Sets:");
    for (nt, fs) in first_sets {
        println!("FIRST({}) = {:?}", nt, fs);
    }
*/