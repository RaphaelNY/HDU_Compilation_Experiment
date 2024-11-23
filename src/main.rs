fn main() {
    let regex = "a(bc)*";
    let nfa = task2::build_nfa_from_regex(regex);
    nfa.print_nfa();

    let dot_output = nfa.to_dot();
    // 假设你已经有函数来写入文件
    task2::write_to_file("nfa.dot", &dot_output).expect("TODO: panic message");

    let mut dfa = nfa.to_dfa();
    let dot_output = dfa.to_dot();
    task2::write_to_file("dfa.dot", &dot_output).unwrap();

    dfa.minimize();
    let dot_output = dfa.to_dot();
    task2::write_to_file("minimized_dfa.dot", &dot_output).unwrap();
}