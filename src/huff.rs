use std::io::Write;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    element: Option<u8>,
    freq: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(elt: Option<u8>, frq: usize) -> Self {
        Self {
            element: elt,
            freq: frq,
            left: None,
            right: None
        }
    }
    pub fn get_elt(&self) -> u8 {
        self.element.unwrap()
    }

    pub fn get_freq(&self) -> usize {
        self.freq
    }
}


pub fn frequency(array: &[u8]) -> HashMap<&u8, usize> {
    let mut h_map = HashMap::new();

    for byte in array {
        let counter = h_map.entry(byte).or_insert(0 as usize);
        *counter += 1;
    }
    h_map
}

pub fn create_tree(array_nodes: &mut Vec<Node>) -> Node {
    while array_nodes.len() > 1 {
        array_nodes.sort_by(|a, b| b.freq.cmp(&a.freq));
        let node0 = array_nodes.pop().unwrap();
        let node1 = array_nodes.pop().unwrap();

        let mut new_node = Node::new(None, node0.freq+node1.freq);
        new_node.left = Some(Box::new(node0));
        new_node.right = Some(Box::new(node1));
        array_nodes.push(new_node);
    }
    array_nodes.pop().unwrap()
}

pub fn encode_element(elt: u8, node: &Node) -> Vec<u8> {
    let mut bits = Vec::new();

    fn walk_through_tree(element: u8, node: &Node, bits: &mut Vec<u8>) -> bool {
        let mut result_left = false;
        let mut result_right = false;

        if matches!(node.element, Some(n) if n == element) {
            true
        } else {
            if let Some(left) = &node.left {
                result_left = walk_through_tree(element, left, bits);
                if result_left {
                    bits.push(0);
                }
            }
            if let Some(right) = &node.right {
                result_right = walk_through_tree(element, right, bits);
                if result_right {
                    bits.push(1);
                }
            }
            result_left || result_right
        }
    }
    walk_through_tree(elt, node, &mut bits);
    bits.reverse();
    bits
}

pub fn save_tree<T: Write>(node: &Node, object: &mut T) {
    // flags
    // 1 - é uma folha, o valor seguinte é o valor dessa folha
    // 2 - é um nó
    // 0 - nulo, não existe qualquer nó
    match node.element {
        Some(element) => {object.write(&[1, element]).unwrap();},
        None          => {object.write(&[2]).unwrap();}
    }
    match &node.left {
        Some(left) => {save_tree(left, object);},
        None       => {object.write(&[0]).unwrap();}
    }
    match &node.right {
        Some(right) => {save_tree(right, object);},
        None        => {object.write(&[0]).unwrap();}
    }
}

pub fn restore_tree(array: &mut Vec<u8>) -> Node {
    fn inner_fn(array: &mut Vec<u8>) -> Option<Box<Node>> {
        match array.pop().unwrap() {
            1 => {
                let mut node = Node::new(Some(array.pop().unwrap()), 0);
                node.left  = inner_fn(array);
                node.right = inner_fn(array);
                Some(Box::new(node))
            },
            2 => {
                let mut node = Node::new(None, 0);
                node.left  = inner_fn(array);
                node.right = inner_fn(array);
                Some(Box::new(node))
            },
            _ => None
        }
    }
    array.reverse();
    let tree = *inner_fn(array).unwrap();
    array.reverse();
    tree
}

pub fn decode_element(bits: &mut Vec<u8>, node: &Node) -> u8 {
    if node.left.is_none() && node.right.is_none() {
        return node.get_elt();
    }
    if bits.pop().unwrap() == 0 {
        return decode_element(bits, node.left.as_ref().unwrap());
    } else {
        return decode_element(bits, node.right.as_ref().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_array_nodes() -> Vec<Node> {
        let mut v_node = Vec::new();
        v_node.push(Node::new(Some(32),  3));
        v_node.push(Node::new(Some(1),   2));
        v_node.push(Node::new(Some(4),   1));
        v_node.push(Node::new(Some(110), 2));
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

    fn example_tree1() -> Node {
        let node0 = Node::new(Some(1),   2);
        let node1 = Node::new(Some(110), 2);
        let node2 = Node::new(Some(32),  3);
        let node3 = Node::new(Some(4),   1);

        let mut node4 = Node::new(None, node3.freq+node1.freq);
        node4.left  = Some(Box::new(node3));
        node4.right = Some(Box::new(node1));
        let mut node5 = Node::new(None, node0.freq+node4.freq);
        node5.left  = Some(Box::new(node0));
        node5.right = Some(Box::new(node4));
        let mut node6 = Node::new(None, node2.freq+node5.freq);
        node6.left  = Some(Box::new(node2));
        node6.right = Some(Box::new(node5));
        node6
    }

    fn example_tree_without_freq() -> Node {
        let node0 = Node::new(Some(1),   0);
        let node1 = Node::new(Some(110), 0);
        let node2 = Node::new(Some(32),  0);
        let node3 = Node::new(Some(4),   0);

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
        let node_root = example_tree1();

        assert_eq!(node_root, create_tree(&mut v_node));
    }

    #[test]
    fn test_encode_element() {
        let node_root = example_tree();

        let v0 = vec![0, 1];
        assert_eq!(v0, encode_element(1, &node_root));

        let v1 = vec![1, 0];
        assert_eq!(v1, encode_element(110, &node_root));
    }

    #[test]
    fn test_decode_element() {
        let node_root = example_tree();

        let mut v_test = vec![0, 1];
        v_test.reverse();
        assert_eq!(1, decode_element(&mut v_test, &node_root));
        let mut v_test = vec![1, 0];
        v_test.reverse();
        assert_eq!(110, decode_element(&mut v_test, &node_root));
    }

    #[test]
    fn test_save_tree() {
        let node_root = example_tree();
        let tree_saved_example = vec![2, 2, 1, 4, 0, 0, 1, 1, 0, 0, 2, 1, 110, 0, 0, 1, 32, 0, 0];

        let mut tree_saved_from_fn = Vec::new();
        save_tree(&node_root, &mut tree_saved_from_fn);

        assert_eq!(tree_saved_example, tree_saved_from_fn);
    }

    #[test]
    fn test_restore_tree() {
        let mut tree_saved_example = vec![2, 2, 1, 4, 0, 0, 1, 1, 0, 0, 2, 1, 110, 0, 0, 1, 32, 0, 0];
        let node_root = example_tree_without_freq();

        assert_eq!(node_root, restore_tree(&mut tree_saved_example));
    }
}
