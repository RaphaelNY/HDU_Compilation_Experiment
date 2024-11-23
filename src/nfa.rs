#[allow(unused)]
struct State {
    pub(crate) id: usize,
    pub(crate) transitions: Vec<Transition>,
    pub(crate) is_accepting: bool,
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
    pub(crate) symbol: Option<char>, // 使用 Option，None 表示 ε 转移
    pub(crate) to_state: usize,
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
    pub(crate) states: Vec<State>,
    pub(crate) start_state: usize,
}

#[allow(unused)]
impl NFA {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn add_state(&mut self, is_accepting: bool) -> usize {
        let new_id = self.states.len();
        let new_state = State {
            id: new_id,
            transitions: Vec::new(),
            is_accepting,
        };
        self.states.push(new_state);
        new_id
    }

    pub(crate) fn add_transition(&mut self, from_state: usize, to_state: usize, symbol: Option<char>) {
        let transition = Transition { symbol, to_state };
        self.states[from_state].transitions.push(transition);
    }

    pub(crate) fn add_epsilon_transition(&mut self, from_state: usize, to_state: usize) {
        let transition = Transition {
            symbol: None, // ε-转移
            to_state,
        };
        self.states[from_state].transitions.push(transition);
    }

    pub(crate) fn accept_state(&self) -> usize {
        self.states.iter().rposition(|s| s.is_accepting).unwrap()
    }

    pub(crate) fn start_state(&self) -> usize {
        self.start_state
    }

    pub(crate) fn set_accept_state(&mut self, state_id: usize) {
        for state in &mut self.states {
            state.is_accepting = false; // 确保只有一个接受状态
        }
        self.states[state_id].is_accepting = true;
    }

    // 复制状态和它们的转移到另一个 NFA
    fn copy_states_and_transitions(&mut self, other: &NFA, offset: usize) {
        for state in other.states.iter() {
            let new_state_id = self.add_state(state.is_accepting);
            for transition in state.transitions.iter() {
                let adjusted_to_state = transition.to_state + offset;
                self.add_transition(new_state_id, adjusted_to_state, transition.symbol.clone());
            }
        }
    }

    // 合并两个 NFA 为一个新的 NFA 使用 | 操作符
    pub(crate) fn union(&mut self, nfa1: &NFA, nfa2: &NFA) {
        let start_state = self.start_state;
        let offset1 = self.states.len();
        self.copy_states_and_transitions(&nfa1, offset1);

        let offset2 = self.states.len();
        self.copy_states_and_transitions(&nfa2, offset2);

        self.add_epsilon_transition(start_state, nfa1.start_state + offset1);
        self.add_epsilon_transition(start_state, nfa2.start_state + offset2);

        let accept_state = self.add_state(true);
        for idx in 0..nfa1.states.len() {
            if nfa1.states[idx].is_accepting {
                self.add_epsilon_transition(idx + offset1, accept_state);
            }
        }
        for idx in 0..nfa2.states.len() {
            if nfa2.states[idx].is_accepting {
                self.add_epsilon_transition(idx + offset2, accept_state);
            }
        }
    }

    pub(crate) fn matches(&self, input: &str) -> bool {
        let mut current_states = vec![self.start_state];
        let mut epsilon_closure = self.epsilon_closure(vec![self.start_state]);

        for c in input.chars() {
            let mut next_states = Vec::new();

            // 只从 epsilon 闭包中的状态转移
            for state_id in &epsilon_closure {
                let state = &self.states[*state_id];
                for transition in &state.transitions {
                    if transition.symbol == Some(c) {
                        // 继续发展 epsilon 闭包
                        let mut temp_closure = self.epsilon_closure(vec![transition.to_state]);
                        next_states.append(&mut temp_closure);
                    }
                }
            }
            // 更新当前状态集合为所有能通过读取当前字符到达的状态的 epsilon 闭包
            epsilon_closure = next_states;
        }

        // 检查任一最终状态是否为接受状态
        epsilon_closure.iter().any(|state_id| self.states[*state_id].is_accepting)
    }

    // Helper function to calculate the epsilon closure of a set of states
    fn epsilon_closure(&self, states: Vec<usize>) -> Vec<usize> {
        let mut closure = states.clone();
        let mut stack = states;

        while let Some(state_id) = stack.pop() {
            let state = &self.states[state_id];
            for transition in &state.transitions {
                if transition.symbol.is_none() && !closure.contains(&transition.to_state) {
                    closure.push(transition.to_state);
                    stack.push(transition.to_state);
                }
            }
        }

        closure
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

    // 生成 DOT 格式的图描述
    pub fn to_dot(&self) -> String {
        let mut dot_graph = String::from("digraph NFA {\n    rankdir=LR;\n    node [shape = circle];\n");

        for state in &self.states {
            if state.is_accepting {
                dot_graph += &format!("    {} [shape = doublecircle];\n", state.id);
            }

            for transition in &state.transitions {
                let label = match transition.symbol {
                    Some(c) => c.to_string(),
                    None => "ε".to_string(),
                };
                dot_graph += &format!("    {} -> {} [label=\"{}\"];\n", state.id, transition.to_state, label);
            }
        }

        dot_graph += "}\n";
        dot_graph
    }
}
