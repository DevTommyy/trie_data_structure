# Trie Data Structure Implementation

## Description

This project provides a Trie data structure implementation in `rust`, designed for efficient storage of words and prefixes and visualization of the `trie`. Tries are useful for tasks like autocomplete, spell checking, and word frequency analysis.

## Features

- Efficient insertion of words.
- Fast searching for words and prefixes.
- Memory-efficient storage of strings.

## Usage

### Installation

```bash
# Clone the repository
git clone https://github.com/DevTommyy/trie_data_structure.git

# Navigate to the project directory
cd trie_data_structure
```

### Running the project

```bash
# Directly running the project
cargo run
# or
# Running the optimized projecet
cargo run --release
```

### Example

```rust
use trie::trie_structure::Trie;

// Creating a new mutable trie
let mut trie: Trie = Trie::create();

// Inserting words into the Trie
trie.insert("hello");
trie.insert("world");

// Searching for words
bool foundHello = trie.contains("hello");
bool foundGoodbye = trie.contains("goodbye");
```

### Example output

```rust
...

// Inserting words into the Trie
trie.insert("apple");
trie.insert("application");
trie.insert("banana");
trie.insert("bat");
trie.insert("cat");
trie.insert("dog");
trie.insert("donut");
trie.insert("zebra");
trie.insert("zoo");

// Visualization into `output.txt` file
trie.visualize();

...
```

```txt
 (Root)
├─│ b
│  └─  a
│     ├─│ t
│     └─  n
│        └─  a
│           └─  n
│              └─  a
├─│ d
│  └─  o
│     ├─│ g
│     └─  n
│        └─  u
│           └─  t
├─│ a
│  └─  p
│     └─  p
│        └─  l
│           ├─│ e
│           └─  i
│              └─  c
│                 └─  a
│                    └─  t
│                       └─  i
│                          └─  o
│                             └─  n
├─│ c
│  └─  a
│     └─  t
└─  z
   ├─│ e
   │  └─  b
   │     └─  r
   │        └─  a
   └─  o
      └─  o

```

## Additional resources

This trie implementation was based on:

- https://drstearns.github.io/tutorials/trie/
