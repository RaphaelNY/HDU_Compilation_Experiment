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

	pub(crate) fn remove_unreachable_states(&mut self) {
        let mut reachable = vec![false; self.states.len()];
        let mut queue = vec![self.start_state];
        
        while let Some(state_id) = queue.pop() {
            if !reachable[state_id] {
                reachable[state_id] = true;
                for &(_, target) in &self.states[state_id].transitions {
                    if !reachable[target] {
                        queue.push(target);
                    }
                }
            }
        }

        self.states = self.states.iter().enumerate()
                                  .filter(|&(id, _)| reachable[id])
                                  .map(|(_, s)| s.clone())
                                  .collect();

        self.accept_states.retain(|&id| reachable[id]);
    }

	fn rebuild_from_partitions(&mut self, partitions: Vec<Vec<usize>>) {
		let mut new_states = Vec::new();
		let mut new_accept_states = Vec::new();
		let mut state_mapping = std::collections::HashMap::new();
	
		// Ensure the initial state is prioritized and mapped to 0
		let initial_partition_index = partitions.iter().position(|p| p.contains(&self.start_state)).unwrap();
		for &old_id in &partitions[initial_partition_index] {
			state_mapping.insert(old_id, 0);
			if self.accept_states.contains(&old_id) && !new_accept_states.contains(&0) {
				new_accept_states.push(0);
			}
		}
	
		// Assign new IDs for other partitions
		let mut new_id = 1;
		for (index, partition) in partitions.iter().enumerate() {
			if index == initial_partition_index { continue; } // Skip the initial partition as it's already processed
	
			for &old_id in partition {
				state_mapping.insert(old_id, new_id);
				if self.accept_states.contains(&old_id) && !new_accept_states.contains(&new_id) {
					new_accept_states.push(new_id);
				}
			}
			new_id += 1;
		}
	
		// Now create new states using the mappings
		for (_index, partition) in partitions.iter().enumerate() {
			let mut transitions = Vec::new();
			for &old_id in partition {
				for &(symbol, target) in &self.states[old_id].transitions {
					if let Some(&new_target_id) = state_mapping.get(&target) {
						transitions.push((symbol, new_target_id));
					} else {
						eprintln!("Error: No mapping found for transition target state {}", target);
					}
				}
			}
			transitions.sort();
			transitions.dedup();
			new_states.push(DFAState { id: *state_mapping.get(&partition[0]).unwrap(), transitions });
		}
	
		self.states = new_states;
		self.accept_states = new_accept_states;
		self.start_state = 0;  // Keep the start state as 0
	}
	

	pub fn minimize(&mut self) {
		// First, remove unreachable states
		self.remove_unreachable_states();
	
		// Initial partition: separate accepting from non-accepting states
		let mut partitions: Vec<Vec<usize>> = vec![Vec::new(), Vec::new()];
		for state in &self.states {
			let partition_index = if self.accept_states.contains(&state.id) { 1 } else { 0 };
			partitions[partition_index].push(state.id);
		}
	
		// Refinement of partitions
		let mut new_partitions = Vec::new();
		let mut stable = false;
		while !stable {
			stable = true;
			new_partitions.clear();
	
			for partition in &partitions {
				let mut local_map: std::collections::HashMap<Vec<Option<usize>>, Vec<usize>> = std::collections::HashMap::new();
				for &state_id in partition {
					let mut key = vec![None; 256]; // Assume ASCII inputs for simplicity
					for &(input, target) in &self.states[state_id].transitions {
						let position = input as usize;
						key[position] = Some(partitions.iter().position(|p| p.contains(&target)).unwrap());
					}
					local_map.entry(key).or_insert_with(Vec::new).push(state_id);
				}
	
				if local_map.len() > 1 {
					stable = false;
				}
	
				for group in local_map.values() {
					new_partitions.push(group.clone());
				}
			}
	
			partitions = new_partitions.clone();
		}
	
		// Create new states from partitions
		self.rebuild_from_partitions(partitions);
	}
	
	// 生成DFA的DOT图描述
	pub fn to_dot(&self) -> String {
		let mut dot_graph = String::from("digraph DFA {\n    rankdir=LR;\n    node [shape = circle];\n");

		// 标记接受状态
		for &accept_state in &self.accept_states {
			dot_graph += &format!("    {} [shape = doublecircle];\n", accept_state);
		}

		// 标记所有状态和转移
		for state in &self.states {
			if !self.accept_states.contains(&state.id) {
				dot_graph += &format!("    {} [shape = circle];\n", state.id);
			}

			for &(symbol, to_state) in &state.transitions {
				let label = symbol.to_string();
				dot_graph += &format!("    {} -> {} [label=\"{}\"];\n", state.id, to_state, label);
			}
		}

		// 特别标记开始状态
		dot_graph += &format!("    {} [color=red];\n", self.start_state);  // 使用红色突出显示开始状态

		dot_graph += "}\n";
		dot_graph
	}
}
