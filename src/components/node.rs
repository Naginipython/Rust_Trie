use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Node {
    pub data: char,
    pub eow: u32,
    pub children: HashMap<char, Node>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(data: char) -> Self {
        Node {
            data,
            ..Default::default()
        }
    }
}