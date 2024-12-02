use task3::grammar::Grammar;
fn main() {
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["AB"], true);
    grammar.add_production("A", vec!["aA", "ε"], false);
    grammar.add_production("B", vec!["b"], false);

    println!("Original Grammar:");
    grammar.display();

    if grammar.is_ll1() {
        println!("The grammar is LL(1)");
    } else {
        println!("The grammar is not LL(1)");
    }

    let table = grammar.create_predictive_parsing_table();
    grammar.display_predictive_parsing_table(&table);


    match grammar.ll1_parse("aab") {
        Ok(_) => println!("Parsing successful"),
        Err(err) => println!("Parsing failed: {}", err),
    }

}

/* 3.1
    let mut grammar = Grammar::new();
    grammar.add_production("S", vec!["S+T", "T"], true);
    grammar.add_production("T", vec!["T*F", "F"]， false);
    grammar.add_production("F", vec!["(E)", "id"], false);

    println!("Original Grammar:");
    grammar.display();

    grammar.eliminate_left_recursion();

    println!("\nGrammar after eliminating left recursion:");
    grammar.display();
*/

/* 3.2
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["apple", "apply", "application", "ball", "bat", "bath", "Xb"], false);
    grammar.add_production("X", vec!["ab", "ac", "ad"], false);

    println!("Original Grammar:");
    grammar.display();

    grammar.eliminate_left_common_factor();
    println!("\nGrammar after eliminating left common factor:");
    grammar.display();
*/

/* 3.3
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["AB"], false);
    grammar.add_production("A", vec!["a", "ε"], false);
    grammar.add_production("B", vec!["b", false]);

    grammar.add_production("S", vec!["AB"], true);
    grammar.add_production("A", vec!["aA", "ε"], false);
    grammar.add_production("B", vec!["b"], false);

    grammar.add_production("E", vec!["T X"], false);
    grammar.add_production("X", vec!["+ T X", "ε"], false);
    grammar.add_production("T", vec!["F Y"], false);
    grammar.add_production("Y", vec!["* F Y", "ε"], false);
    grammar.add_production("F", vec!["( E )", "d"], false);

    grammar.add_production("P", vec!["i E t S", "a"], false);
    grammar.add_production("E", vec!["b"], false);
    grammar.add_production("T", vec![";"], false);
    grammar.add_production("S", vec!["a S e", "ε"], false);

    grammar.add_production("Q", vec!["R Q'"], false);
    grammar.add_production("Q'", vec!["+ R Q'", "ε"], false);
    grammar.add_production("R", vec!["c R'"], false);
    grammar.add_production("R'", vec!["* c R'", "ε"], false);


    println!("Original Grammar:");
    grammar.display();

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
*/

/*
    let mut grammar = Grammar::new();

    grammar.add_production("S", vec!["AB"], true);
    grammar.add_production("A", vec!["aA", "ε"], false);
    grammar.add_production("B", vec!["b"], false);

    println!("Original Grammar:");
    grammar.display();

    if grammar.is_ll1() {
        println!("The grammar is LL(1)");
    } else {
        println!("The grammar is not LL(1)");
    }

    let table = grammar.create_predictive_parsing_table();
    grammar.display_predictive_parsing_table(&table);


    match grammar.ll1_parse("aab") {
        Ok(_) => println!("Parsing successful"),
        Err(err) => println!("Parsing failed: {}", err),
    }
*/