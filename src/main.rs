fn main() {
    loop{
        println!("chosse task or quit\n0. quit\n1. task");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();
        match input {
            0 => return,
            1 => {
                let regex = task2::input(); // "a(bc)*"
                let nfa = task2::build_nfa_from_regex(&*regex);
                nfa.print_nfa();
                task2::dot(nfa);
            }

            _ => {
                println!("Invalid input");
            }
        }

    }
}