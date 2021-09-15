use std::{fs::File, io::Write};

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    elemento: Option<u8>,
    freq: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(elt: Option<u8>, frq: u32) -> Self {
        Self {
            elemento: elt,
            freq: frq,
            left: None,
            right: None
        }
    }
    pub fn get_elt(&self) -> u8 {
        self.elemento.unwrap()
    }

    pub fn get_freq(&self) -> u32 {
        self.freq
    }
}


pub fn frequency(array: &mut Vec<u8>) -> Vec<Node> {
    let mut array_nodes: Vec<Node> = Vec::new();

    while array.len() != 0 {
        let elt = array[0];
        let frq = array.iter().filter(|&n| *n == elt).count();
        array_nodes.push(Node::new(Some(elt), frq as u32));

        array.retain(|value| *value != elt);
    }

    array_nodes
}

pub fn create_tree(array_nodes: &mut Vec<Node>) -> Node {
    while array_nodes.len() > 1 {
        array_nodes.sort_by_key(|k| k.freq);
        let node0 = array_nodes.remove(0);
        let node1 = array_nodes.remove(0);

        let mut new_node = Node::new(None, node0.freq+node1.freq);
        new_node.left = Some(Box::new(node0));
        new_node.right = Some(Box::new(node1));
        array_nodes.push(new_node);
    }
    array_nodes.remove(0)
}

fn walk_through_tree(elt: u8, node: &Node, bits: &mut Vec<u8>) -> bool {
    let mut valid_path = false;
    let mut result_left = false;
    let mut result_right = false;

    if let Some(n) = node.elemento {
        if n == elt {
            valid_path = true;
        }
    } else {
        if let Some(left) = &node.left {
            result_left = walk_through_tree(elt, left, bits);
            if result_left {
                bits.insert(0, 0);
            }
        }
        if let Some(right) = &node.right {
            result_right = walk_through_tree(elt, right, bits);
            if result_right {
                bits.insert(0, 1);
            }
        }
        if result_left || result_right {
            valid_path = true;
        } else {
            valid_path = false;
        }
    }
    valid_path
}

pub fn encode_elt(elt: u8, node: &Node) -> Vec<u8> {
    let mut bits = Vec::new();
    walk_through_tree(elt, node, &mut bits);
    bits
}

pub fn tree_to_file(node: &Node, file: &mut File) {
    if let Some(c) = node.elemento {
        file.write(&[c, 32]).unwrap();
    } else {
        file.write(&[32, 110]).unwrap();
    }

    if let Some(left) = &node.left {
        tree_to_file(left, file);
    } else {
        file.write(&[32, 35]).unwrap();
    }

    if let Some(right) = &node.right {
        tree_to_file(right, file);
    } else {
        file.write(&[32, 35]).unwrap();
    }
}

pub fn file_to_tree(array: &mut Vec<u8>) -> Option<Node> {
    if array[1] == 32 {
        let mut node = Node::new(Some(array[0]), 0);
        array.drain(0..2);

        let no0 = file_to_tree(array);
        if let Some(n0) = no0 {
            node.left = Some(Box::new(n0));
        } else {
            array.drain(0..2);
        }

        let no1 = file_to_tree(array);
        if let Some(n1) = no1 {
            node.right = Some(Box::new(n1));
        } else {
            array.drain(0..2);
        }

        return Some(node);

    } else if array[1] == 110 {
        let mut node = Node::new(None, 0);
        array.drain(0..2);

        let no0 = file_to_tree(array);
        if let Some(n0) = no0 {
            node.left = Some(Box::new(n0));
        } else {
            array.drain(0..2);
        }

        let no1 = file_to_tree(array);
        if let Some(n1) = no1 {
            node.right = Some(Box::new(n1));
        } else {
            array.drain(0..2);
        }

        return Some(node);
    }
    None
}

pub fn decode_elt(bits: &mut Vec<u8>, node: &Node) -> u8 {
    if node.left.is_none() && node.right.is_none() {
        return node.get_elt();
    }
    if bits[0] == 0 {
        bits.remove(0);
        return decode_elt(bits, node.left.as_ref().unwrap());
    } else {
        bits.remove(0);
        return decode_elt(bits, node.right.as_ref().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency() {
        let mut v = vec![32, 32, 32, 1, 4, 1, 110, 110];
        let v_node = vec![
            Node::new(Some(32), 3),
            Node::new(Some(1), 2),
            Node::new(Some(4), 1),
            Node::new(Some(110), 2),
        ];

        assert_eq!(v_node, frequency(&mut v));
    }
}
