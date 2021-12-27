use std::io::Write;
use std::collections::HashMap;

pub enum Tree {
    Node {
        left: Box<Tree>,
        right: Box<Tree>,
        freq: usize
    },
    Leaf {
        element: u8,
        freq: usize
    }
}

impl Tree {
    pub fn new_leaf(element: u8, freq: usize) -> Tree {
        Self::Leaf { element, freq }
    }

    fn get_freq(&self) -> usize {
        match self {
            &Tree::Leaf { freq, .. } => freq,
            &Tree::Node { freq, .. } => freq,
        }
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

pub fn create_tree(elements: &HashMap<&u8, usize>) -> Tree {
    let mut nodes = Vec::with_capacity(elements.len());
    for (k, v) in elements {
        nodes.push(Tree::new_leaf(**k, *v));
    }

    fn tree(nodes: &mut Vec<Tree>) {
        if nodes.len() > 1 {
            nodes.sort_by(|a, b| b.get_freq().cmp(&a.get_freq()));
            let node0 = nodes.pop().unwrap();
            let node1 = nodes.pop().unwrap();

            let root = Tree::Node {
                freq: node0.get_freq() + node1.get_freq(),
                left: Box::new(node0),
                right: Box::new(node1),
            };
            nodes.push(root);

            tree(nodes);
        }
    }

    tree(&mut nodes);
    nodes.pop().unwrap()
}

pub fn encode_element(elt: u8, node: &Tree) -> Vec<u8> {
    let mut bits = Vec::new();

    fn walk_through_tree(element_target: u8, node: &Tree, bits: &mut Vec<u8>) -> bool {
        match node {
            Tree::Leaf { element, .. } => {*element == element_target},
            Tree::Node { left, right, .. } => {
                let result_left = walk_through_tree(element_target, left, bits);
                if result_left {
                    bits.push(0);
                }

                let result_right = walk_through_tree(element_target, right, bits);
                if result_right {
                    bits.push(1);
                }
                result_left || result_right
            }
        }
    }
    walk_through_tree(elt, node, &mut bits);
    bits.reverse();
    bits
}

pub fn save_tree<T: Write>(node: &Tree, object: &mut T) {
    // flags
    // 1 - é uma folha, o valor seguinte é o valor dessa folha
    // 2 - é um nó
    // 0 - nulo, não existe qualquer nó

    match node {
        Tree::Leaf {element, .. } => {object.write(&[1, *element]).unwrap();},
        Tree::Node {left, right, ..} => {
            object.write(&[2]).unwrap();
            save_tree(left, object);
            save_tree(right, object);
        }
    }
}

/*
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
*/

#[cfg(test)]
mod tests {
    use super::*;
}
