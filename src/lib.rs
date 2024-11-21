#[allow(unused)]
struct State {
    id: usize,
    transitions: Vec<Transition>,
    is_accepting: bool,
}

#[allow(unused)]
struct Transition {
    symbol: Option<char>, // 使用 Option，None 表示 ε 转移
    to_state: usize,
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
            '*' => {}, // Implement closure operation
            '|' => {}, // Implement union operation
            '.' => {}, // Implement concatenation
            _ => {
                // Handle single character, simplest NFA: a single transition to a new state
                let mut nfa = NFA::new();
                let end_state = nfa.add_state(true);
                nfa.add_transition(0, end_state, Some(c));
                nfa_stack.push(nfa);
            },
        }
    }

    // At the end, there should be exactly one NFA on the stack
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
    fn test_nfa_ab_star() {
        let regex = "ab*";
        let nfa = build_nfa_from_regex(regex);
        
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
        }
}
