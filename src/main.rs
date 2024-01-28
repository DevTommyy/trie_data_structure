use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    value: Option<char>, // option cause the root is None
    is_final: bool,
    child_nodes: HashMap<char, TrieNode>,
}

#[derive(Debug)]
struct Trie {
    root_node: TrieNode,
}

impl TrieNode {
    pub fn new(ch: char, is_final: bool) -> TrieNode {
        TrieNode {
            value: Some(ch),
            is_final,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            value: None,
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }
    /// Creates and inserts a new `TrieNode` given the letter to insert
    /// and if it is the final letter of a word or no
    pub fn insert_value(&mut self, ch: char, is_final: bool) {
        self.child_nodes.insert(ch, TrieNode::new(ch, is_final));
    }
}

impl Trie {
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
    pub fn insert(&mut self, string_val: String) {
        // TODO improve this function with iterators
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        // iterate over every character of the word to insert
        for letter_counter in 0..char_list.len() {
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                // if the character is already present in the map of childs
                // move the cursor on it
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                // else save the index of the iteration in which the
                // new char to insert was found
                last_match = letter_counter;
                break;
            }
            // increment that index (cause the actual index is + 1)
            last_match = letter_counter + 1;
        }

        // if the last match was the last char of the Vec
        // it means the is_final flag is true
        if last_match == char_list.len() {
            current_node.is_final = true;
        } else {
            // or else we have to iterate over the remainder of the word to insert every new character
            for new_counter in last_match..char_list.len() {
                // this print is for debug purposes
                // println!(
                //     "Inserting {} into {}",
                //     char_list[new_counter],
                //     current_node.value.unwrap_or_default()
                // );
                // inserts into the child the new value
                current_node.insert_value(char_list[new_counter], false);
                // moves the cursor to the newly inserted child
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            // for the final child inserted it changes the is_final flag to true
            current_node.is_final = true;
        }
    }

    // TODO understand the logic behind thisd printing, cause this is made by chatGPT

    // exposed function, it uses a private helper function that does the actual logic
    pub fn visualize(&self) {
        Trie::visualize_recursive(&self.root_node, 0, String::new(), false);
    }

    fn visualize_recursive(node: &TrieNode, level: usize, prefix: String, is_last: bool) {
        // Print the current node's value
        if let Some(value) = node.value {
            println!(
                "{}{}{} {}",
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
            );
        } else {
            // i think this can be made in just
            // println!("{} (Root)", prefix);
            println!(
                "{}{}{} (Root)",
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
                }
            );
        }

        // Recursively visualize child nodes
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
            Trie::visualize_recursive(child_node, level + 1, new_prefix.clone(), is_last_child);
        }
    }
}

fn main() {
    let mut trie = Trie::create();
    trie.insert(String::from("hello"));
    trie.insert(String::from("world"));
    trie.insert(String::from("hi"));
    trie.visualize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert() {
        let mut trie = Trie::create();
        trie.insert(String::from("hello"));
        trie.insert(String::from("world"));
        trie.insert(String::from("hi"));
        println!("{:?}", trie);
    }

    #[test]
    fn test_trie_visualize() {
        let mut trie = Trie::create();
        trie.insert(String::from("apple"));
        trie.insert(String::from("apples"));
        trie.insert(String::from("application"));
        trie.insert(String::from("apply"));
        trie.insert(String::from("applying"));
        trie.insert(String::from("apparatus"));
        trie.insert(String::from("apprentice"));
        trie.insert(String::from("approach"));
        trie.insert(String::from("approaching"));
        trie.insert(String::from("approve"));

        trie.visualize();
    }
}
