use std::collections::HashMap;

/// 定义 Trie 节点
#[derive(Default)]
pub(crate) struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    /// 插入候选式到 Trie
    pub(crate) fn insert(&mut self, word: &str) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::default);
        }
        current.is_end = true;
    }

    /// 提取公共前缀并生成新的规则
    pub(crate) fn extract_common_prefix(
        &self,
        parent_nt: &str,
    ) -> (HashMap<String, Vec<String>>, Vec<String>) {
        let mut new_productions = HashMap::new();
        let mut new_non_terminals = Vec::new();
        let mut prefix = vec![];

        self.collect_common_prefix(parent_nt, &mut prefix, &mut new_productions, &mut new_non_terminals);
        (new_productions, new_non_terminals)
    }

    /// 遍历 Trie，提取最长公共前缀
    fn collect_common_prefix(
        &self,
        parent_nt: &str,
        prefix: &mut Vec<char>,
        new_productions: &mut HashMap<String, Vec<String>>,
        new_non_terminals: &mut Vec<String>,
    ) {
        // 当前节点是分支点或结束节点
        if self.children.len() > 1 || self.is_end {
            let prefix_string: String = prefix.iter().collect(); // 当前公共前缀
            let new_nt = format!("{}'", parent_nt); // 生成新非终结符

            let mut candidates = vec![];

            // 遍历子节点，收集子规则
            for (&ch, child) in &self.children {
                let mut child_prefix = vec![ch];
                child.collect_candidates(&mut candidates, &mut child_prefix);
            }

            // 如果当前路径是结束路径，添加 ε
            if self.is_end {
                candidates.push("ε".to_string());
            }

            // 添加子规则到新非终结符，合并候选式
            new_productions
                .entry(new_nt.clone())
                .or_insert_with(Vec::new)
                .extend(candidates);

            // 记录新非终结符
            new_non_terminals.push(new_nt.clone());

            // 父规则添加 公共前缀 + 新非终结符，合并父规则
            new_productions
                .entry(parent_nt.to_string())
                .or_insert_with(Vec::new)
                .push(format!("{}{}", prefix_string, new_nt));
        } else {
            // 递归处理子节点
            for (&ch, child) in &self.children {
                prefix.push(ch);
                child.collect_common_prefix(parent_nt, prefix, new_productions, new_non_terminals);
                prefix.pop();
            }
        }
    }

    /// 收集路径下的所有候选式
    fn collect_candidates(&self, candidates: &mut Vec<String>, prefix: &mut Vec<char>) {
        if self.is_end {
            candidates.push(prefix.iter().collect());
        }
        for (&ch, child) in &self.children {
            prefix.push(ch);
            child.collect_candidates(candidates, prefix);
            prefix.pop();
        }
    }

    /// 打印当前 Trie 树（用于调试）
    pub(crate) fn display(&self, depth: usize, prefix: String) {
        if self.is_end {
            println!("{}(End)", " ".repeat(depth) + &prefix);
        }
        for (&ch, child) in &self.children {
            let new_prefix = format!("{}{}", prefix, ch);
            child.display(depth + 2, new_prefix);
        }
    }
}
