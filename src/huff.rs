use std::collections::VecDeque;
use std::{fs::File, io::Write};

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    elemento: Option<u8>,
    freq: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(elt: Option<u8>, frq: usize) -> Self {
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

    pub fn get_freq(&self) -> usize {
        self.freq
    }
}


pub fn frequency(array: &mut Vec<u8>) -> VecDeque<Node> {
    let mut array_nodes: VecDeque<Node> = VecDeque::new();
    while array.len() != 0 {
        let elt = array[0];

        let prev_len = array.len();
        array.retain(|value| *value != elt);
        let freq = prev_len - array.len();

        array_nodes.push_back(Node::new(Some(elt), freq));
    }
    array_nodes
}

pub fn create_tree(array_nodes: &mut VecDeque<Node>) -> Node {
    while array_nodes.len() > 1 {
        array_nodes.make_contiguous().sort_by_key(|k| k.freq);
        let node0 = array_nodes.pop_front().unwrap();
        let node1 = array_nodes.pop_front().unwrap();

        let mut new_node = Node::new(None, node0.freq+node1.freq);
        new_node.left = Some(Box::new(node0));
        new_node.right = Some(Box::new(node1));
        array_nodes.push_back(new_node);
    }
    array_nodes.pop_front().unwrap()
}

pub fn encode_element(elt: u8, node: &Node) -> VecDeque<u8> {
    let mut bits = VecDeque::new();

    fn walk_through_tree(elt: u8, node: &Node, bits: &mut VecDeque<u8>) -> bool {
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
                    bits.push_front(0);
                }
            }
            if let Some(right) = &node.right {
                result_right = walk_through_tree(elt, right, bits);
                if result_right {
                    bits.push_front(1);
                }
            }
            valid_path = result_left || result_right;
        }
        valid_path
    }
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

    fn example_array_nodes() -> VecDeque<Node> {
        let mut v_node = VecDeque::new();
        v_node.push_back(Node::new(Some(32),  3));
        v_node.push_back(Node::new(Some(1),   2));
        v_node.push_back(Node::new(Some(4),   1));
        v_node.push_back(Node::new(Some(110), 2));
        v_node
    }

    #[test]
    fn test_frequency() {
        let mut v = vec![32, 32, 32, 1, 4, 1, 110, 110];
        let v_node = example_array_nodes();

        assert_eq!(v_node, frequency(&mut v));
    }

    fn example_tree() -> Node {
        let node0 = Node::new(Some(1),   2);
        let node1 = Node::new(Some(110), 2);
        let node2 = Node::new(Some(32),  3);
        let node3 = Node::new(Some(4),   1);

        let mut node4 = Node::new(None, node3.freq+node0.freq);
        node4.left  = Some(Box::new(node3));
        node4.right = Some(Box::new(node0));
        let mut node5 = Node::new(None, node1.freq+node2.freq);
        node5.left  = Some(Box::new(node1));
        node5.right = Some(Box::new(node2));
        let mut node6 = Node::new(None, node4.freq+node5.freq);
        node6.left  = Some(Box::new(node4));
        node6.right = Some(Box::new(node5));
        node6
    }

    #[test]
    fn test_create_tree() {
        let mut v_node = example_array_nodes();
        let node_root = example_tree();

        assert_eq!(node_root, create_tree(&mut v_node));
    }

    #[test]
    fn test_encode_element() {
        let node_root = example_tree();

        let v0 = VecDeque::from(vec![0, 1]);
        assert_eq!(v0, encode_element(1, &node_root));

        let v1 = VecDeque::from(vec![1, 0]);
        assert_eq!(v1, encode_element(110, &node_root));
    }
}
