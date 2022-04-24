use rustc_hash::FxHashMap;
use std::io::Write;
use unchecked_unwrap::UncheckedUnwrap;

#[derive(PartialEq, Debug)]
pub enum Tree {
    Node {
        left: Box<Tree>,
        right: Box<Tree>,
        freq: usize,
    },
    Leaf {
        element: u8,
        freq: usize,
    },
}

impl Tree {
    fn get_freq(&self) -> usize {
        match *self {
            Tree::Leaf { freq, .. } => freq,
            Tree::Node { freq, .. } => freq,
        }
    }
}

pub fn frequency(array: &[u8]) -> FxHashMap<u8, usize> {
    let mut h_map = FxHashMap::default();
    for byte in array.iter().copied() {
        let counter = h_map.entry(byte).or_insert(0_usize);
        *counter += 1;
    }
    h_map
}

pub fn create_tree(elements: &FxHashMap<u8, usize>) -> Tree {
    let mut nodes = Vec::with_capacity(elements.len());
    for (k, v) in elements {
        nodes.push(Tree::Leaf {
            element: *k,
            freq: *v,
        });
    }

    fn tree(nodes: &mut Vec<Tree>) {
        if nodes.len() > 1 {
            nodes.sort_by(|a, b| b.get_freq().cmp(&a.get_freq()));
            let node0 = unsafe { nodes.pop().unchecked_unwrap() };
            let node1 = unsafe { nodes.pop().unchecked_unwrap() };

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
    unsafe { nodes.pop().unchecked_unwrap() }
}

pub fn encode_element(elt: u8, node: &Tree) -> Vec<u8> {
    let mut bits = Vec::new();

    fn walk_through_tree(element_target: u8, node: &Tree, bits: &mut Vec<u8>) -> bool {
        match node {
            Tree::Leaf { element, .. } => *element == element_target,
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

pub fn save_tree<T: Write>(node: &Tree, writer: &mut T) {
    // flags
    // 1 - é uma folha, o valor seguinte é o valor dessa folha
    // 2 - é um nó

    match node {
        Tree::Leaf { element, .. } => {
            writer.write_all(&[1, *element]).unwrap();
        }
        Tree::Node { left, right, .. } => {
            writer.write_all(&[2]).unwrap();
            save_tree(left, writer);
            save_tree(right, writer);
        }
    }
}

pub fn restore_tree(array: &mut impl Iterator<Item = u8>) -> Tree {
    match array.next().unwrap() {
        1 => Tree::Leaf {
            element: array.next().unwrap(),
            freq: 0,
        },
        2 => Tree::Node {
            left: Box::new(restore_tree(array)),
            right: Box::new(restore_tree(array)),
            freq: 0,
        },
        _ => Tree::Leaf {
            element: 0,
            freq: 0,
        },
    }
}

pub fn decode_element(bits: &mut impl Iterator<Item = u8>, node: &Tree) -> u8 {
    match node {
        Tree::Leaf { element, .. } => *element,
        Tree::Node { left, right, .. } => {
            if bits.next().unwrap() == 0 {
                decode_element(bits, left)
            } else {
                decode_element(bits, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tree_example() -> Tree {
        // [97, 110, 97, 103, 114, 97, 109];
        Tree::Node {
            left: Box::new(Tree::Leaf {
                element: 97,
                freq: 3,
            }),
            right: Box::new(Tree::Node {
                left: Box::new(Tree::Node {
                    left: Box::new(Tree::Leaf {
                        element: 114,
                        freq: 1,
                    }),
                    right: Box::new(Tree::Leaf {
                        element: 109,
                        freq: 1,
                    }),
                    freq: 2,
                }),
                right: Box::new(Tree::Node {
                    left: Box::new(Tree::Leaf {
                        element: 110,
                        freq: 1,
                    }),
                    right: Box::new(Tree::Leaf {
                        element: 103,
                        freq: 1,
                    }),
                    freq: 2,
                }),
                freq: 4,
            }),
            freq: 7,
        }
    }

    #[test]
    fn test_create_tree() {
        let array_file = [97, 110, 97, 103, 114, 97, 109];
        let hash_file = frequency(&array_file);
        let tree = create_tree(&hash_file);

        assert_eq!(tree, tree_example());
    }
}
