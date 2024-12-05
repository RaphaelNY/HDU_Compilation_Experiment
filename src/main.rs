use std::io;
use task3::grammar::Grammar;
fn main() {
    loop {
        println!("would you like to input your own grammar or use the default grammar?");
        println!("0. input your own grammar");
        println!("1. use the task3_1 default grammar");
        println!("2. use the task3_2 default grammar");
        println!("3. use the task3_3 default grammar");
        println!("4. use the task3_4 default grammar");
        println!("6");
        println!("7");
        println!("5. quit");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        let num: u32 = num.trim().parse().expect("Please type a number!");
        let mut grammar = Grammar::new();

        match num {
            0 => {
                let (non_terminals, productions) = task3::input();
                task3::add_prod(&mut grammar, non_terminals, productions);
                println!("witch task would you like to run?");
                println!("1. eliminate left recursion");
                println!("2. eliminate left common factor");
                println!("3. find first and follow sets");
                println!("4. check if the grammar is LL(1)");

                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read line");
                let task: u32 = task.trim().parse().expect("Please type a number!");
                match task {
                    1 => task3::task3_1(&mut grammar),
                    2 => task3::task3_2(&mut grammar),
                    3 => task3::task3_3(&mut grammar),
                    4 => task3::task3_4(&mut grammar),
                    _ => println!("Please type a number between 1 and 4"),
                }
            },
            1 => {
                grammar.add_production("S", vec!["S+T", "T"], false);
                grammar.add_production("T", vec!["T*F", "F"], false);
                grammar.add_production("F", vec!["(E)", "id"], false);
                task3::task3_1(&mut grammar);
            },
            2 => {
                grammar.add_production("S", vec!["apple", "apply", "application", "ball", "bat", "bath", "Xb"], false);
                grammar.add_production("X", vec!["ab", "ac", "ad"], false);
                task3::task3_2(&mut grammar);
            },
            3 => {
                grammar.add_production("S", vec!["AB"], true);
                grammar.add_production("A", vec!["a", "ε"], false);
                grammar.add_production("B", vec!["b"], false);
                task3::task3_3(&mut grammar);
            },
            4 => {
                grammar.add_production("S", vec!["AB"], true);
                grammar.add_production("A", vec!["aA", "ε"], false);
                grammar.add_production("B", vec!["b"], false);
                task3::task3_4(&mut grammar);
            },
            5 => return,
            6 => {
                grammar.add_production("S", vec!["MH", "a"], true);
                grammar.add_production("H", vec!["LSo", "ε"], false);
                grammar.add_production("K", vec!["dML","ε"], false);
                grammar.add_production("L", vec!["eHf"], false);
                grammar.add_production("M", vec!["K","bLM"], false);
                task3::task3_3(&mut grammar);
            }
            7 => {
                grammar.add_production("S", vec!["a", "^", "(T)"], true);
                grammar.add_production("T", vec!["T,S", "S"], false);
                task3::task3_4_1(&mut grammar);
            }
            _ => println!("Please type a number between 0 and 5"),
        }
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

/* 3.4
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

/*
    println!("would you like to input your own grammar or use the default grammar?");
    println!("0. input your own grammar");
    println!("1. use the task3_1 default grammar");
    println!("2. use the task3_2 default grammar");
    println!("3. use the task3_3 default grammar");
    println!("4. use the task3_4 default grammar");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: u32 = num.trim().parse().expect("Please type a number!");
    let mut grammar = Grammar::new();

    match num {
        0 => {
            let (non_terminals, productions) = task3::input();
            task3::add_prod(&mut grammar, non_terminals, productions);
            println!("witch task would you like to run?");
            println!("1. eliminate left recursion");
            println!("2. eliminate left common factor");
            println!("3. find first and follow sets");
            println!("4. check if the grammar is LL(1)");
            let mut task = String::new();
            io::stdin().read_line(&mut task).expect("Failed to read line");
            let task: u32 = task.trim().parse().expect("Please type a number!");
            match task {
                1 => task3::task3_1(&mut grammar),
                2 => task3::task3_2(&mut grammar),
                3 => task3::task3_3(&mut grammar),
                4 => task3::task3_4(&mut grammar),
                _ => println!("Please type a number between 1 and 4"),
            }
        },
        1 => {
            grammar.add_production("S", vec!["S+T", "T"], false);
            grammar.add_production("T", vec!["T*F", "F"], false);
            grammar.add_production("F", vec!["(E)", "id"], false);
            task3::task3_1(&mut grammar);
        },
        2 => {
            grammar.add_production("S", vec!["apple", "apply", "application", "ball", "bat", "bath", "Xb"], false);
            grammar.add_production("X", vec!["ab", "ac", "ad"], false);
            task3::task3_2(&mut grammar);
        },
        3 => {
            grammar.add_production("S", vec!["AB"], false);
            grammar.add_production("A", vec!["a", "ε"], false);
            grammar.add_production("B", vec!["b"], false);
            task3::task3_3(&mut grammar);
        },
        4 => {
            grammar.add_production("S", vec!["AB"], true);
            grammar.add_production("A", vec!["aA", "ε"], false);
            grammar.add_production("B", vec!["b"], false);
            task3::task3_4(&mut grammar);
        },
        _ => println!("Please type a number between 0 and 4"),
    }
*/