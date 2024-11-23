use task2_1::regex_to_postfix;
use task2_1::build_nfa_from_postfix;
use task2_1::write_to_file;
fn main() {
    let regex = "ab(c|2)*";
    let postfix = regex_to_postfix(regex);
    println!("Postfix: {}", postfix);
    let nfa = build_nfa_from_postfix(&postfix);
    nfa.print_nfa();

    let dot_output = nfa.to_dot();
    // 假设你已经有函数来写入文件
    write_to_file("nfa.dot", &dot_output).expect("TODO: panic message");

}