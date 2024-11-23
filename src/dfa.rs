#[derive(Debug, Clone)]
pub struct DFA {
	pub states: Vec<DFAState>,
	pub start_state: usize,
	pub accept_states: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct DFAState {
	pub id: usize,
	pub transitions: Vec<(char, usize)>, // (符号, 到达状态的索引)
}

impl DFA {
	pub fn new() -> Self {
		Self {
			states: Vec::new(),
			start_state: 0,
			accept_states: Vec::new(),
		}
	}

	// 添加状态和转移的方法
	// 其他必要的方法
}
