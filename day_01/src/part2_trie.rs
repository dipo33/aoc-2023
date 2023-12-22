use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_of_word: bool,
    num_value: Option<u8>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            end_of_word: false,
            num_value: None,
        }
    }

    fn add(&mut self, word: &str, value: u8) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }
        node.end_of_word = true;
        node.num_value = Some(value);
    }
}

fn get_trie_from_digit_map() -> TrieNode {
    let mut trie = TrieNode::new();
    trie.add("one", 1);
    trie.add("two", 2);
    trie.add("three", 3);
    trie.add("four", 4);
    trie.add("five", 5);
    trie.add("six", 6);
    trie.add("seven", 7);
    trie.add("eight", 8);
    trie.add("nine", 9);
    trie
}

#[inline(always)]
fn store_digits(digit: u8, first_digit: &mut Option<u8>, last_digit: &mut Option<u8>) {
    if first_digit.is_none() {
        *first_digit = Some(digit);
    } else {
        *last_digit = Some(digit);
    }
}

fn calibrate<'a>(str: &str, trie: &'a TrieNode, nodes: &mut Vec<&'a TrieNode>) -> u8 {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;

    let mut node: &TrieNode;
    for c in str.chars() {
        if c.is_ascii_digit() {
            store_digits(c.to_digit(10).unwrap() as u8, &mut first_digit, &mut last_digit);
            continue;
        }

        for i in (0..nodes.len()).rev() {
            node = nodes[i];
            match node.children.get(&c) {
                Some(next) => {
                    if next.end_of_word {
                        store_digits(next.num_value.unwrap(), &mut first_digit, &mut last_digit);
                        nodes.remove(i);
                    } else {
                        nodes[i] = next;
                    }
                }
                None => {
                    nodes.remove(i);
                }
            }
        }

        if let Some(next) = trie.children.get(&c) {
            nodes.push(next);
        }
    }
    nodes.clear();

    if last_digit.is_none() {
        last_digit = first_digit;
    }

    first_digit.unwrap() * 10 + last_digit.unwrap()
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let trie = get_trie_from_digit_map();
    let mut temp_nodes: Vec<&TrieNode> = Vec::with_capacity(2);

    let result: u32 = contents
        .split_whitespace()
        .map(|str| calibrate(str, &trie, &mut temp_nodes) as u32)
        .sum();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}