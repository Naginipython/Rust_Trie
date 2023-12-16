use std::collections::VecDeque;
use super::node::Node;

#[derive(Debug)]
pub struct Trie {
    pub root: Box<Node>,
    pub size: u32,
}

#[allow(dead_code)]
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Box::new(Node::new('\0')),
            size: 1,
        }
    }

    pub fn add(&mut self, str: &str) {
        let mut new_str = String::new();
        for c in str.to_lowercase().chars() {
            if c.is_alphabetic() {
                new_str.push(c);
            } else if c == '\'' {
                // purposefully ignored
            } else {
                new_str.push(' ');
            }
        }

        for word in new_str.split_whitespace() {
            let mut char_arr: VecDeque<char> = word.chars().collect();
            check_nodes(&mut self.root, &mut char_arr, &mut self.size);
        }

        fn create_nodes(node: &mut Node, chars: &mut VecDeque<char>, size: &mut u32) {
            if chars.len() > 0 {
                *size += 1;
                let l = chars.pop_front().unwrap();
                node.children.insert(l, Node::new(l));
                create_nodes(
                    node.children.get_mut(&l).unwrap(), 
                    chars,
                    size
                );
            } else {
                node.eow += 1;
            }
        }

        fn check_nodes(node: &mut Node, chars: &mut VecDeque<char>, size: &mut u32) {
            if chars.len() > 0 {
                let l = chars.pop_front().unwrap();
                if node.children.contains_key(&l) {
                    check_nodes(node.children.get_mut(&l).unwrap(), chars, size);
                } else {
                    chars.push_front(l);
                    create_nodes(node, chars, size);
                }
            } else {
                node.eow += 1;
            }
        }
    }

    pub fn find(&self, str: &str) -> bool {
        fn helper(node: &Node, chars: &mut VecDeque<char>) -> bool {
            if chars.len() > 0 {
                let l = chars.pop_front().unwrap();
                if node.children.contains_key(&l) {
                    return helper(node.children.get(&l).unwrap(), chars);
                } else {
                    return false;
                }
            } else {
                return node.eow > 0;
            }
        }
        let mut char_arr: VecDeque<char> = str.chars().collect();
        helper(&self.root, &mut char_arr)
    }

    pub fn find_amount(&self, str: &str) -> u32 {
        fn helper(node: &Node, chars: &mut VecDeque<char>) -> u32 {
            if chars.len() > 0 {
                let l = chars.pop_front().unwrap();
                if node.children.contains_key(&l) {
                    return helper(node.children.get(&l).unwrap(), chars);
                } else {
                    return 0;
                }
            } else {
                return node.eow;
            }
        }
        let mut char_arr: VecDeque<char> = str.chars().collect();
        helper(&self.root, &mut char_arr)
    }
}