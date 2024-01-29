use std::collections::HashMap;

#[derive(Debug)]
/// Represents the node element of the trie.
///
/// a `Trie` contains multiple `TrieNode`s
pub struct TrieNode {
    pub value: Option<char>, // option cause the root is None
    pub is_final: bool,
    pub child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    /// creates a new `TrieNode` entity that is NOT a root
    pub fn new(ch: char, is_final: bool) -> TrieNode {
        TrieNode {
            value: Some(ch),
            is_final,
            child_nodes: HashMap::new(),
        }
    }

    /// creates a new `TrieNode` representing the root
    pub fn new_root() -> TrieNode {
        TrieNode {
            value: None,
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }
    /// Creates and inserts a new `TrieNode` into
    /// the `HashMap` of its children given the letter to insert
    /// and if it is the final letter of a word or no
    ///
    /// # Params
    ///
    /// - `ch` the `char` to insert in the child map
    /// - `is_final` a `bool` flag stating if the letter is final or not
    pub fn insert_value(&mut self, ch: char, is_final: bool) {
        self.child_nodes.insert(ch, TrieNode::new(ch, is_final));
    }

    /// Checks if the letter passed is the same as the value of the `TrieNode`
    ///
    /// # Params
    ///
    /// `ch` a `char` representing the letter to check for the equality
    ///
    /// # Returns
    ///
    /// `true` if the char is equal, `false` if it's not
    pub fn equals(&self, ch: char) -> bool {
        self.value.is_some_and(|val| val == ch)
    }
}
