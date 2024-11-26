use std::collections::{HashMap, HashSet};

/// 定义 Trie 节点
#[derive(Default)]
pub(crate) struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
    productions: Vec<String>,
}

/// 实现 Trie 树的插入和提取逻辑
impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::default);
        }
        current.is_end = true;
        current.productions.push(word.to_string());
    }

    fn extract_common_prefix(
        &self,
        parent_nt: &str,
    ) -> (HashMap<String, Vec<String>>, Vec<String>) {
        let mut new_productions = HashMap::new();
        let mut new_non_terminals = Vec::new();
        let mut current = vec![];

        self.collect_common_prefix(parent_nt, &mut current, &mut new_productions, &mut new_non_terminals);
        (new_productions, new_non_terminals)
    }

	fn collect_common_prefix(
        &self,
        parent_nt: &str,
        prefix: &mut Vec<char>,
        new_productions: &mut HashMap<String, Vec<String>>,
        new_non_terminals: &mut Vec<String>,
    ) {
        // 如果当前节点是公共前缀的分支点或结束节点
        if self.children.len() > 1 || self.is_end {
            let prefix_string: String = prefix.iter().collect();
            let new_nt = format!("{}'", parent_nt);
            let mut new_candidates = vec![];

            // 为每个子分支收集候选式
            for (&ch, child) in &self.children {
                let mut child_prefix = prefix.clone();
                child_prefix.push(ch);

                let mut child_candidates = vec![];
                child.collect_candidates(&mut child_candidates, &child_prefix);
                new_candidates.push(format!("{}{}", ch, new_nt));

                // 添加新的非终结符规则
                new_productions.insert(new_nt.clone(), child_candidates);
            }

            // 如果当前路径有直接的终止候选式
            if self.is_end {
                new_candidates.push("ε".to_string());
            }

            // 更新父节点的候选式
            new_productions
                .entry(parent_nt.to_string())
                .or_insert_with(Vec::new)
                .push(format!("{}{}", prefix_string, new_nt));

            new_non_terminals.push(new_nt);
        } else {
            // 继续递归处理公共前缀
            for (&ch, child) in &self.children {
                prefix.push(ch);
                child.collect_common_prefix(parent_nt, prefix, new_productions, new_non_terminals);
                prefix.pop();
            }
        }
    }

    /// 收集所有候选式
    fn collect_candidates(&self, candidates: &mut Vec<String>, prefix: &Vec<char>) {
        if self.is_end {
            candidates.push(prefix.iter().collect());
        }
        for (&ch, child) in &self.children {
            let mut child_prefix = prefix.clone();
            child_prefix.push(ch);
            child.collect_candidates(candidates, &child_prefix);
        }
    }
}