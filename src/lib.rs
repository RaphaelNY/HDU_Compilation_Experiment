mod nfa;
mod dfa;

use nfa::*;
use std::fs::File;
use std::io;
use std::io::Write;


fn process_operator(operators: &mut Vec<char>, output: &mut String, op: char, precedence: fn(char) -> i32) {
    while !operators.is_empty() && precedence(*operators.last().unwrap()) >= precedence(op) {
        output.push(operators.pop().unwrap());
    }
    operators.push(op);
}

pub fn regex_to_postfix(regex: &str) -> String {
    let mut output = String::new();
    let mut operators = Vec::new();
    let mut last_was_operand_or_closure = false;

    let precedence = |op: char| {
        match op {
            '*' => 3,
            '.' => 2,  // 显式连接操作符
            '|' => 1,
            '(' => 0,
            _ => -1,
        }
    };

    for c in regex.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                if last_was_operand_or_closure {
                    // We add a concatenation operator only if the last character was an operand or a closure
                    process_operator(&mut operators, &mut output, '.', precedence);
                }
                output.push(c);
                last_was_operand_or_closure = true;
            },
            '(' => {
                if last_was_operand_or_closure {
                    // Treat implicit concatenation
                    process_operator(&mut operators, &mut output, '.', precedence);
                }
                operators.push(c);
                last_was_operand_or_closure = false;
            },
            ')' => {
                while *operators.last().unwrap() != '(' {
                    output.push(operators.pop().unwrap());
                }
                operators.pop(); // Remove '('
                last_was_operand_or_closure = true; // After closing a parenthesis, we can have a concatenation
            },
            '*' => {
                process_operator(&mut operators, &mut output, c, precedence);
                // Continue allowing concatenation after a closure
                last_was_operand_or_closure = true;
            },
            '|' => {
                process_operator(&mut operators, &mut output, c, precedence);
                last_was_operand_or_closure = false;
            },
            _ => {},
        }
    }

    while !operators.is_empty() {
        output.push(operators.pop().unwrap());
    }

    output
}

pub fn build_nfa_from_postfix(postfix: &str) -> NFA {
    let mut nfa_stack: Vec<NFA> = Vec::new();

    for c in postfix.chars() {
        match c {
            '*' => {
                let nfa = nfa_stack.pop().unwrap();
                let mut closure_nfa = NFA::new();

                // 创建新的初始和接受状态
                let new_start_state = closure_nfa.start_state;

                // 复制原始 NFA 的所有状态到 closure_nfa，并更新转移的索引
                let mut state_id_map = vec![0; nfa.states.len()];
                for (index, state) in nfa.states.iter().enumerate() {
                    let new_state_id = closure_nfa.add_state(state.is_accepting);
                    state_id_map[index] = new_state_id;
                }

                // 复制转移
                for (index, state) in nfa.states.iter().enumerate() {
                    for trans in &state.transitions {
                        let symbol = trans.symbol.clone();
                        closure_nfa.add_transition(state_id_map[index], state_id_map[trans.to_state], symbol);
                    }
                }

                let new_accept_state = closure_nfa.add_state(true);
                // 连接新的起始状态到原始 NFA 的起始状态，并允许ε-转移直接到新的接受状态
                closure_nfa.add_epsilon_transition(new_start_state, state_id_map[nfa.start_state()]);
                closure_nfa.add_epsilon_transition(new_start_state, new_accept_state);

                // 从原始 NFA 的接受状态连接到新的接受状态，并添加ε-转移从原始接受状态到原始起始状态（支持重复匹配）
                let old_accept = state_id_map[nfa.accept_state()];
                closure_nfa.add_epsilon_transition(old_accept, new_accept_state);
                closure_nfa.add_epsilon_transition(old_accept, state_id_map[nfa.start_state()]);

                // 设置新的起始状态和接受状态
                closure_nfa.start_state = new_start_state;
                closure_nfa.set_accept_state(new_accept_state);

                // 最终，推回 NFA 栈
                nfa_stack.push(closure_nfa);
            },// have corrected
            '|' => {
                let nfa2 = nfa_stack.pop().unwrap();
                let nfa1 = nfa_stack.pop().unwrap();
                let mut union_nfa = NFA::new();
                union_nfa.union(&nfa1, &nfa2);
                nfa_stack.push(union_nfa);
            },// have corrected
            '.' => {
                let nfa2 = nfa_stack.pop().unwrap();
                let mut nfa1 = nfa_stack.pop().unwrap();

                // 获取 nfa1 的接受状态
                let nfa1_accept_state = nfa1.states.iter().position(|s| s.is_accepting).unwrap();
                nfa1.states[nfa1_accept_state].is_accepting = false; // 将 nfa1 的接受状态标记为非接受状态

                // 计算 nfa2 中状态的偏移量
                let offset = nfa1.states.len() - 1;  // 调整偏移量以合并到 nfa1 的最后一个状态

                // 遍历 nfa2 的所有状态和转移，正确添加到 nfa1
                for state in nfa2.states {
                    let new_state_id = if state.id == nfa2.start_state {
                        nfa1_accept_state  // 如果是 nfa2 的起始状态，则合并到 nfa1 的最后一个状态
                    } else {
                        nfa1.add_state(state.is_accepting)  // 否则创建新状态
                    };

                    for transition in state.transitions {
                        nfa1.add_transition(new_state_id, transition.to_state + offset, transition.symbol);
                    }
                }

                nfa_stack.push(nfa1);
            },// have corrected
            _ => {
                let mut nfa = NFA::new();
                let end_state = nfa.add_state(true);
                nfa.add_transition(0, end_state, Some(c));
                nfa_stack.push(nfa);
            },
        }
    }

    nfa_stack.pop().unwrap()
}

pub fn build_nfa_from_regex(regex: &str) -> NFA {
	let postfix = regex_to_postfix(regex);
	build_nfa_from_postfix(&postfix)
}

pub fn write_to_file(filename: &str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}


pub fn input() -> String {
    println!("Enter a regular expression:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn dot(nfa: NFA) {
    let dot_output = nfa.to_dot();
    // 假设你已经有函数来写入文件
    write_to_file("nfa.dot", &dot_output).expect("TODO: panic message");

    let mut dfa = nfa.to_dfa();
    let dot_output = dfa.to_dot();
    write_to_file("dfa.dot", &dot_output).unwrap();

    dfa.minimize();
    let dot_output = dfa.to_dot();
    write_to_file("minimized_dfa.dot", &dot_output).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nda_ab() {
        let regex = "ab";
        let postfix = regex_to_postfix(regex);
        println!("Postfix: {}", postfix);
        let nfa = build_nfa_from_regex(&postfix);

        assert!(nfa.matches("ab"));
        assert!(!nfa.matches("a"));
        assert!(!nfa.matches("b"));
    }

    #[test]
    fn test_nfa_ab_star() {
        let regex = "ab*";
        let postfix = regex_to_postfix(regex);
        println!("Postfix: {}", postfix);
        let nfa = build_nfa_from_regex(&postfix);
        nfa.print_nfa();
        
        assert!(nfa.matches("a"));
        assert!(nfa.matches("ab"));
        assert!(nfa.matches("abb"));
        assert!(!nfa.matches("b"));
    }

    #[test]
    fn test_nfa_a_or_b() {
        let regex = "a|b";
        let nfa = build_nfa_from_regex(regex);
        nfa.print_nfa();

        assert!(nfa.matches("a"));
        assert!(nfa.matches("b"));
        assert!(!nfa.matches("ab"));
        assert!(!nfa.matches("ba"));
    }

    #[test]
    fn test_nfa_ab_parentheses_star() {
        let regex = "(ab)*";
        let nfa = build_nfa_from_regex(regex);
        
        assert!(nfa.matches("abab"));
        assert!(nfa.matches("ab"));
        assert!(!nfa.matches("a"));
        assert!(!nfa.matches("baba"));
    }

    #[test]
    fn test_nfa_a_bc_star() {
        let regex = "a(b|c)*";
        let nfa = build_nfa_from_regex(regex);
        nfa.print_nfa();

        assert!(nfa.matches("abcbcbc"));
        assert!(nfa.matches("a"));
        assert!(nfa.matches("abc"));
        assert!(nfa.matches("acb"));
        assert!(!nfa.matches("ca"));
    }

    #[test]
    fn test_nfa_abc() {
        let regex = "abc";
        let nfa = build_nfa_from_regex(regex);
        nfa.print_nfa();
        
        assert!(nfa.matches("abc"));
        assert!(!nfa.matches("ab"));
        assert!(!nfa.matches("bc"));
        assert!(!nfa.matches("abcabc"));
    }
	
	// 中缀转后缀测试代码
    #[test]
    fn test_regex_to_postfix() {
        assert_eq!(regex_to_postfix("ab"), "ab.");
        assert_eq!(regex_to_postfix("a|b"), "ab|");
        assert_eq!(regex_to_postfix("a|b*"), "ab*|");
        assert_eq!(regex_to_postfix("a(b|c)*"), "abc|*.");
        assert_eq!(regex_to_postfix("(a|b|c)*"), "ab|c|*");
        }
}
