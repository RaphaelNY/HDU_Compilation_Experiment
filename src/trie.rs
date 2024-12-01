use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

#[derive(Default)]
pub(crate) struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
    count: usize, // 用于计数经过此节点的单词数
    depth: usize, // 节点深度
}

impl Trie {
    // 创建一个新的 Trie 树
    pub fn new() -> Self {
        Self { root: TrieNode::new(0) }
    }

    // 插入单词到 Trie 树中
    #[allow(unused)]
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for (i, c) in word.chars().enumerate() {
            node = node.children.entry(c).or_insert_with(|| TrieNode::new(node.depth + 1));
            node.count += 1;  // 增加经过此节点的单词计数
        }
        node.is_end = true;  // 标记单词结束
    }

    // 提取最长公共前缀及其分支
    pub fn extract_longest_common_prefix(&self) -> Vec<(String, Vec<String>)> {
        let mut results = Vec::new();
        let current_node = &self.root;
        let mut stack = vec![(current_node, "".to_string())]; // 用栈来追踪节点和当前前缀

        while let Some((node, prefix)) = stack.pop() {
            // 判断节点是否是一个有效的分支点
            if node.children.len() > 1 || (node.is_end && node.children.len() > 0) {
                let mut branches = Vec::new();
                // 检查所有子节点
                for (&c, _child_node) in &node.children {
                    branches.push(format!("{}{}", prefix, c));
                }
                let has_end_child = node.children.values().any(|child| child.is_end);
                if node.is_end || has_end_child {
                    // 如果当前节点是结束节点或有多个子节点，记录当前前缀并不再进一步递归
                    results.push((prefix.clone(), branches));
                    continue;
                }
                // 继续探索所有子节点
                for (&c, child_node) in &node.children {
                    stack.push((child_node, format!("{}{}", prefix, c)));
                }
            } else if node.children.len() == 1 {
                // 只有一个子节点并且不是结束点，继续沿这一路径递归
                let (&c, next_node) = node.children.iter().next().unwrap();
                if !node.is_end {
                    stack.push((next_node, format!("{}{}", prefix, c)));
                } else {
                    // 如果是结束点但无其他子节点，不再继续
                    results.push((format!("{}{}", prefix, c), vec![]));
                }
            }
        }

        results
    }

    // 实现 Display trait 来美观打印 Trie
    pub fn display(&self) -> String {
        self.root.display("", false, String::new())
    }
}

impl TrieNode {
    // 初始化一个新节点
    pub(crate) fn new(depth: usize) -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
            count: 0,
            depth,
        }
    }

    // 辅助函数，用于递归显示 Trie
    fn display(&self, prefix: &str, last: bool, indent: String) -> String {
        let mut result = String::new();

        // 当前节点的前缀处理
        let current_prefix = if last { "`-- " } else { "|-- " };
        let new_indent = if last { "    " } else { "|   " };

        result.push_str(&format!("{}{}{}", indent, current_prefix, prefix));
        if self.is_end {
            result.push_str(" (end)");
        }
        result.push_str(&format!(" [{}]\n", self.count));

        let mut children = self.children.iter().collect::<Vec<(&char, &TrieNode)>>();
        children.sort_by_key(|&(c, _)| c); // 对子节点排序，确保输出一致性

        for (i, (char, node)) in children.iter().enumerate() {
            let last_child = i == children.len() - 1;
            result.push_str(&node.display(&char.to_string(), last_child, format!("{}{}", indent, new_indent)));
        }

        result
    }
}

// 使用 Display 实现
impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}