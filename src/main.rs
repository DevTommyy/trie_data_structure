use std::time;

mod trie;
use trie::trie_structure::Trie;

// for the sake of testing in this exemple is used an input file with 10k italian words
fn main() {
    let start = time::SystemTime::now();
    let mut trie = Trie::create();

    let contents =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    contents.lines().for_each(|line| trie.insert(line));

    trie.visualize();

    println!("{}", trie.contains("achille"));
    let elapsed = start.elapsed().unwrap();
    println!("{:?}", elapsed);
}
