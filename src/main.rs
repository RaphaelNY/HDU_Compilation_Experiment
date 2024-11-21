use task2_1::regex_to_postfix;
use task2_1::build_nfa_from_postfix;
fn main() {
    let regex = "(ab)*";
    let postfix = regex_to_postfix(regex);
    println!("Postfix: {}", postfix);
    let nfa = build_nfa_from_postfix(&postfix);
    nfa.print_nfa();
}