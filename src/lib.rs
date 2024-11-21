#[allow(unused)]
struct State {
    id: usize,
    transitions: Vec<Transition>,
    is_accepting: bool,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            transitions: self.transitions.clone(),
            is_accepting: self.is_accepting,
        }
    }
}

#[allow(unused)]
struct Transition {
    symbol: Option<char>, // 使用 Option，None 表示 ε 转移
    to_state: usize,
}

impl Clone for Transition {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol,
            to_state: self.to_state,
        }
    }
}

#[allow(unused)]
pub struct NFA {
    states: Vec<State>,
    start_state: usize,
}

#[allow(unused)]
impl NFA {
    fn new() -> Self {
        let start_state = State {
            id: 0,
            transitions: Vec::new(),
            is_accepting: false,
        };

        Self {
            states: vec![start_state],
            start_state: 0,
        }
    }

    fn add_state(&mut self, is_accepting: bool) -> usize {
        let new_id = self.states.len();
        let new_state = State {
            id: new_id,
            transitions: Vec::new(),
            is_accepting,
        };
        self.states.push(new_state);
        new_id
    }

    fn add_transition(&mut self, from_state: usize, to_state: usize, symbol: Option<char>) {
        let transition = Transition { symbol, to_state };
        self.states[from_state].transitions.push(transition);
    }

    fn add_epsilon_transition(&mut self, from_state: usize, to_state: usize) {
        let transition = Transition {
            symbol: None, // ε-转移
            to_state,
        };
        self.states[from_state].transitions.push(transition);
    }

    fn accept_state(&self) -> usize {
        self.states.iter().rposition(|s| s.is_accepting).unwrap()
    }

    fn start_state(&self) -> usize {
        self.start_state
    }

    fn set_accept_state(&mut self, state_id: usize) {
        for state in &mut self.states {
            state.is_accepting = false; // 确保只有一个接受状态
        }
        self.states[state_id].is_accepting = true;
    }
    
	fn matches(&self, input: &str) -> bool {
		let mut current_states = vec![self.start_state];

		for c in input.chars() {
			let mut next_states = Vec::new();
			for state_id in current_states {
				let state = &self.states[state_id];
				for transition in &state.transitions {
					if transition.symbol == Some(c) {
						next_states.push(transition.to_state);
					}
				}
			}
			current_states = next_states;
		}

		current_states.iter().any(|&state_id| self.states[state_id].is_accepting)
	}

    pub fn print_nfa(&self) {
        println!("NFA States and Transitions:");
        for state in &self.states {
            // 打印每个状态的信息
            let state_type = if state.is_accepting { "Accepting" } else { "Normal" };
            println!("State {}: {}", state.id, state_type);

            // 打印从这个状态出发的所有转移
            if state.transitions.is_empty() {
                println!("  No transitions");
            } else {
                for transition in &state.transitions {
                    let symbol = match transition.symbol {
                        Some(c) => c.to_string(),
                        None => "ε".to_string(), // 用 ε 表示空转移
                    };
                    println!("  Transition on '{}': to State {}", symbol, transition.to_state);
                }
            }
        }
    }
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
                    while !operators.is_empty() && precedence(*operators.last().unwrap()) >= precedence('.') {
                        output.push(operators.pop().unwrap());
                    }
                    operators.push('.');
                }
                output.push(c);
                last_was_operand_or_closure = true;
            },
            '(' => {
                if last_was_operand_or_closure {
                    // Treat implicit concatenation
                    while !operators.is_empty() && precedence(*operators.last().unwrap()) >= precedence('.') {
                        output.push(operators.pop().unwrap());
                    }
                    operators.push('.');
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
                while !operators.is_empty() && precedence(*operators.last().unwrap()) >= precedence(c) {
                    output.push(operators.pop().unwrap());
                }
                operators.push(c);
                // Continue allowing concatenation after a closure
                last_was_operand_or_closure = true;
            },
            '|' => {
                while !operators.is_empty() && precedence(*operators.last().unwrap()) >= precedence(c) {
                    output.push(operators.pop().unwrap());
                }
                operators.push(c);
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
                let new_start_state = closure_nfa.add_state(false);
                let new_accept_state = closure_nfa.add_state(true);

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
            },


            '|' => {
                let nfa2 = nfa_stack.pop().unwrap();
                let nfa1 = nfa_stack.pop().unwrap();
                let mut union_nfa = NFA::new();

                let start_state = union_nfa.add_state(false);
                let accept_state = union_nfa.add_state(true);

                // 复制 nfa1 到 union_nfa 并更新转移
                let nfa1_offset = union_nfa.states.len();
                for state in nfa1.states.iter() {
                    let new_state_id = union_nfa.add_state(state.is_accepting);
                    for transition in state.transitions.iter() {
                        union_nfa.add_transition(new_state_id, transition.to_state + nfa1_offset, transition.symbol);
                    }
                }

                // 复制 nfa2 到 union_nfa 并更新转移
                let nfa2_offset = union_nfa.states.len();
                for state in nfa2.states.iter() {
                    let new_state_id = union_nfa.add_state(state.is_accepting);
                    for transition in state.transitions.iter() {
                        union_nfa.add_transition(new_state_id, transition.to_state + nfa2_offset, transition.symbol);
                    }
                }

                // 添加从新起始状态到 nfa1 和 nfa2 起始状态的 ε-转移
                union_nfa.add_epsilon_transition(start_state, nfa1.start_state() + nfa1_offset);
                union_nfa.add_epsilon_transition(start_state, nfa2.start_state() + nfa2_offset);

                // 从 nfa1 和 nfa2 的接受状态添加 ε-转移到新的接受状态
                for idx in 0..nfa1.states.len() {
                    if nfa1.states[idx].is_accepting {
                        union_nfa.add_epsilon_transition(idx + nfa1_offset, accept_state);
                    }
                }
                for idx in 0..nfa2.states.len() {
                    if nfa2.states[idx].is_accepting {
                        union_nfa.add_epsilon_transition(idx + nfa2_offset, accept_state);
                    }
                }

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
        
        assert!(nfa.matches("abcbcbc"));
        assert!(nfa.matches("a"));
        assert!(nfa.matches("abc"));
        assert!(!nfa.matches("acb"));
    }

    #[test]
    fn test_nfa_abc() {
        let regex = "abc|";
        let nfa = build_nfa_from_regex(regex);
        
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
