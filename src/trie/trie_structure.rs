use std::{fs::File, io::Write};

use super::trie_node::*;

#[derive(Debug)]
/// Represents the trie data structure
pub struct Trie {
    root_node: TrieNode,
}

impl Trie {
    /// Instanciate a new `Trie`
    pub fn create() -> Trie {
        Trie {
            root_node: TrieNode::new_root(),
        }
    }

    // cases that could present:
    // 1. Empty Trie
    // 2. No Overlap
    // 3. String Already Present
    // 4. Partial Overlap
    //
    // The first two are straigh forward.
    // In both cases new branches are created from the root
    //
    // For the other two cases:
    //  - 3. if the whole string is already present the only thing that happens
    //        is the last char being marked as final
    //  - 4. the string is inserted char by char from the last char present to the
    //        last new char (which is then marked as final)

    /// Inserts a new word into the `Trie`, letter by lettere
    ///
    /// # Params
    /// `word` a `&str` representing the word to insert
    pub fn insert(&mut self, word: &str) {
        let mut cursor = &mut self.root_node;

        for c in word.chars() {
            // ff the character is not present, insert a new node.
            if !cursor.child_nodes.contains_key(&c) {
                cursor.insert_value(c, false);
            }
            // move to the next node.
            cursor = cursor.child_nodes.get_mut(&c).unwrap();
        }

        // Mark the last node as final.
        cursor.is_final = true;
    }

    /// Checks if a word is present in the tree
    ///
    /// # Params
    ///
    /// `word` the `&str` to check for
    ///
    /// # Returns
    ///
    /// `true` if the word is preesent, `false` if it's not
    pub fn contains(&self, word: &str) -> bool {
        // probably could be improved with iterators
        let char_list: Vec<char> = word.chars().collect();
        let mut cursor_option = self.root_node.child_nodes.get(&char_list[0]);

        for char_idx in 0..char_list.len() {
            if let Some(cursor) = cursor_option {
                if cursor.equals(char_list[char_idx]) {
                    if cursor.is_final && char_idx == char_list.len() - 1 {
                        return true;
                    } else {
                        cursor_option = cursor.child_nodes.get(&char_list[char_idx + 1]);
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        false
    }

    /// Prints the `Trie` into an `output.txt` file
    ///
    /// # Panics
    ///
    /// If there is any error in the io the method panics
    // exposed method, it uses a private helper function that does the actual logic
    pub fn visualize(&self) {
        let mut file = File::create("output.txt").expect("failed to create file");
        Trie::visualize_recursive(&self.root_node, 0, String::new(), false, &mut file);
    }

    // util function
    /// Recursively prints the `Trie` to a file
    ///
    /// # Params
    ///
    /// - `node` the current `TrieNode` to visualize
    /// - `level` an `usize` representing the depth
    /// - `prefix` a `String` representing the prefix to print before the letter
    /// - `is_last` a `bool` flag stating if it's the last letter or not
    /// - `file` a `File` of the file in which to print the output
    fn visualize_recursive(
        node: &TrieNode,
        level: usize,
        prefix: String,
        is_last: bool,
        file: &mut File,
    ) {
        // Print the current node's value
        if let Some(value) = node.value {
            file.write_all(
                format!(
                    "{}{}{} {}\n",
                    prefix,
                    if level == 0 {
                        ""
                    } else if is_last {
                        "└─"
                    } else {
                        "├─"
                    },
                    if level == 0 {
                        ""
                    } else if is_last {
                        " "
                    } else {
                        "│"
                    },
                    value
                )
                .as_bytes(),
            )
            .expect("failed to write to file");
        } else {
            file.write_all(format!("{} (Root)\n", prefix).as_bytes())
                .expect("failed to write to file");
        }

        // recursively visualize child nodes
        let child_count = node.child_nodes.len();
        let mut new_prefix = prefix.clone();
        new_prefix.push_str(if level == 0 {
            ""
        } else if is_last {
            "   "
        } else {
            "│  "
        });

        for (index, (_key, child_node)) in node.child_nodes.iter().enumerate() {
            // this is just checking if the index is equal to the numbers of children - 1 cause if it is
            // it means it is the last
            let is_last_child = index == child_count - 1;
            // recursively printing each node
            Trie::visualize_recursive(
                child_node,
                level + 1,
                new_prefix.clone(),
                is_last_child,
                file,
            );
        }
    }
}
