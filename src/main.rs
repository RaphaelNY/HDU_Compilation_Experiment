fn main() {
    let regex = task2::input(); // "a(bc)*"
    let nfa = task2::build_nfa_from_regex(&*regex);
    nfa.print_nfa();
    task2::dot(nfa);
}