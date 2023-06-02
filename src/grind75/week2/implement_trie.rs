// Problem: https://leetcode.com/problems/implement-trie-prefix-tree/

use std::collections::HashMap;

#[allow(dead_code)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_word: bool,
}

#[allow(dead_code)]
impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_word: false,
        }
    }
}

#[allow(dead_code)]
struct Trie {
    root: TrieNode,
}

#[allow(dead_code)]
impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert(TrieNode::new());
            current_node = next_node;
        }
        current_node.is_end_word = true;
    }

    fn search_prefix_node(&self, word: String) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return None,
            }
        }

        Some(current_node)
    }

    fn search(&self, word: String) -> bool {
        let current_node = self.search_prefix_node(word);

        match current_node {
            Some(current_node) => current_node.is_end_word,
            None => false,
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.search_prefix_node(prefix).is_some()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }

    #[test]
    fn example_2() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        trie.insert("application".to_string());
        trie.insert("hello".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(trie.search("hello".to_string()));
        assert!(trie.starts_with("appl".to_string()));
        assert!(trie.starts_with("appli".to_string()));
        assert!(trie.starts_with("hel".to_string()));
        assert!(!trie.search("appli".to_string()));
        assert!(!trie.search("hel".to_string()));
    }
}
