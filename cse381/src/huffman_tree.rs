use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use bitvec::prelude::*;

#[derive(Debug)]
pub struct Tree {
    count : u32,
    root : Box<Node>,
}

#[derive(Debug)]
enum Node {
    Support(Box<Node>, Box<Node>),
    Leaf(char)
}


impl Eq for Tree {
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min Heap
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// To ensure that we have deterministic huffman codes, the profile
// will be a vector of (letter,count) sorted by the letter.  Note
// that the letter is always unique.
pub fn profile(text : &str) -> Vec<(char, u32)> {
    let mut result = HashMap::new();
    text.chars().for_each(|c| *result.entry(c).or_insert(0) += 1);
    let mut result : Vec<(char, u32)> = result.into_iter().collect();
    result.sort();
    result
}

pub fn build_tree(profiled_text : &[(char, u32)]) -> Option<Tree> {
    if profiled_text.is_empty() {
        return None;
    }

    let mut queue: BinaryHeap<Tree> = BinaryHeap::new();
    for (letter, count) in profiled_text.iter() {
        let tree = Tree { 
            count: *count, 
            root: Box::new(Node::Leaf(*letter)) 
        };
        queue.push(tree);
    }
    while queue.len() > 1 {
        let n1 = queue.pop().unwrap();
        let n2 = queue.pop().unwrap();
        let tree = Tree { 
            count: n1.count + n2.count, 
            root: Box::new(Node::Support(n1.root, n2.root))
        };
        queue.push(tree);
    }

    Some(queue.pop().unwrap())
}

pub fn create_encoding(tree : &Option<Tree>) -> HashMap<char,BitVec> {
    let mut result = HashMap::new();

    if let Some(tree) = tree {
        create_encoding_(&tree.root, BitVec::new(), &mut result);
    }

    result
}

fn create_encoding_(node : &Node, bit_vec : BitVec, mapping : &mut HashMap<char,BitVec>) {
    match node {
        Node::Leaf(letter) => {
            let _= if bit_vec.is_empty() {
                mapping.insert(*letter, bitvec![0])
            } 
            else {
                mapping.insert(*letter, bit_vec)
            };
        }
        Node::Support(left, right) => {
            let mut left_bit_vec = bit_vec.clone();
            let mut right_bit_vec = bit_vec.clone();
            left_bit_vec.push(false);
            right_bit_vec.push(true);
            create_encoding_(left, left_bit_vec, mapping);
            create_encoding_(right, right_bit_vec, mapping);
        }
    }
}

pub fn encode(text : &str, encoding : &HashMap<char,BitVec>) -> Option<BitVec> {
    let mut result = BitVec::new();
    for letter in text.chars() {
        match encoding.get(&letter) {
            Some(letter) => result.extend(letter),
            None => return None
        }
    }
    Some(result)
}

pub fn decode(bits : &BitVec, tree : &Option<Tree>) -> Option<String> {
    // Special Case - Nothing to decode - OK
    if bits.is_empty() {
        return Some(String::new());
    }

    // Special Case - Nothing to decode with - NOK
    let root = match tree {
        Some(tree) => tree.root.as_ref(),
        None => return None
    };

    let mut result = String::new();
    let mut curr = root;
    
    let mut at_root = true;

    for bit in bits {
        curr = match curr {
            Node::Leaf(_) => {
                // Special Case - Tree with only one Node
                // It must be encoded as 0
                if *bit {
                    return None;
                }
                curr
            }
            Node::Support(left, right) => {
                at_root = false;
                if *bit { right } else { left }
            }
        };
        if let Node::Leaf(letter) = curr {
            result.push(*letter);
            at_root = true;
            curr = root;
        } 
    }

    if at_root {
        return Some(result);
    }
    // Speial Case - No match for the last bits - NOK
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_profile() {
        let text = "the rain in spain stays mainly in the plan";
        let profiled_text = profile(text);
        assert_eq!(profiled_text,
            vec![(' ', 8), ('a', 5), ('e', 2), ('h', 2),
                 ('i', 5), ('l', 2), ('m', 1), ('n', 6),
                 ('p', 2), ('r', 1), ('s', 3), ('t', 3),
                 ('y', 2)]
        );
    }

    #[test]
    fn test2_build_tree() {
        let profiled_text = 
            vec![(' ', 8), ('a', 5), ('e', 2), ('h', 2),
                 ('i', 5), ('l', 2), ('m', 1), ('n', 6),
                 ('p', 2), ('r', 1), ('s', 3), ('t', 3),
                 ('y', 2)];
        let tree = build_tree(&profiled_text);
        assert!(tree.is_some());
        assert_eq!(tree.unwrap().count, 42);
    }


    #[test]
    fn test3_create_encoding() {
        let tree = Some(Tree {count: 12, root :
            Box::new(Node::Support(
                Box::new(Node::Support(
                    Box::new(Node::Leaf('d')),
                    Box::new(Node::Leaf('b')),
                )),
                Box::new(Node::Support(
                    Box::new(Node::Support(
                        Box::new(Node::Leaf('e')),
                        Box::new(Node::Leaf('a'))
                    )),
                    Box::new(Node::Leaf('c'))
                )),
            )) 
        });
        let encoding = create_encoding(&tree);
        let expected = vec![
            ('a',bitvec![1,0,1]),
            ('b',bitvec![0,1]),
            ('c',bitvec![1,1]),
            ('d',bitvec![0,0]),
            ('e',bitvec![1,0,0])].into_iter().collect();
        assert_eq!(encoding, expected);
    }

    #[test]
    fn test4_create_encoding_empty() {
        let encoding = create_encoding(&None);
        assert_eq!(encoding, HashMap::new());
    }

    #[test]
    fn test5_encode() {
        let encoding_map = vec![
            ('a',bitvec![1,0,1]),
            ('b',bitvec![0,1]),
            ('c',bitvec![1,1]),
            ('d',bitvec![0,0]),
            ('e',bitvec![1,0,0])].into_iter().collect();
        let text = "abcde";
        let encoded_text = encode(text, &encoding_map);
        assert!(encoded_text.is_some());
        assert_eq!(encoded_text.unwrap(),bitvec![1,0,1,0,1,1,1,0,0,1,0,0]);
    }

    #[test]
    fn test6_encode_invalid() {
        let encoding_map = vec![
            ('a',bitvec![1,0,1]),
            ('b',bitvec![0,1]),
            ('c',bitvec![1,1]),
            ('d',bitvec![0,0]),
            ('e',bitvec![1,0,0])].into_iter().collect();
        let text = "abczde";
        let encoded_text = encode(text, &encoding_map);
        assert!(encoded_text.is_none());
    }

    #[test]
    fn test7_decode() {
        let tree = Some(Tree {count: 12, root :
            Box::new(Node::Support(
                Box::new(Node::Support(
                    Box::new(Node::Leaf('d')),
                    Box::new(Node::Leaf('b')),
                )),
                Box::new(Node::Support(
                    Box::new(Node::Support(
                        Box::new(Node::Leaf('e')),
                        Box::new(Node::Leaf('a'))
                    )),
                    Box::new(Node::Leaf('c'))
                )),
            )) 
        });
        let encoded_text = bitvec![1,0,1,0,1,1,1,0,0,1,0,0];
        let decoded_text = decode(&encoded_text, &tree);
        assert!(decoded_text.is_some());
        assert_eq!(decoded_text.unwrap(),"abcde");
    }

    #[test]
    fn test8_decode_invalid() {
        let tree = Some(Tree {count: 12, root :
            Box::new(Node::Support(
                Box::new(Node::Support(
                    Box::new(Node::Leaf('d')),
                    Box::new(Node::Leaf('b')),
                )),
                Box::new(Node::Support(
                    Box::new(Node::Support(
                        Box::new(Node::Leaf('e')),
                        Box::new(Node::Leaf('a'))
                    )),
                    Box::new(Node::Leaf('c'))
                )),
            )) 
        });
        let encoded_text = bitvec![1,0,1,0,1,1,1,0,0,1,1,1];
        let decoded_text = decode(&encoded_text, &tree);
        assert!(decoded_text.is_none());
    }

    #[test]
    fn test9_decode_invalid_single() {
        let tree = Some(Tree {count: 12, root :
            Box::new(Node::Leaf('a'))
        });
        let encoded_text = bitvec![0,0,0,0,1,0,0,0,0,0];
        let decoded_text = decode(&encoded_text, &tree);
        assert!(decoded_text.is_none());
    }

    #[test]
    fn test10_encode_decode() {
        let text = "the rain in spain stays mainly in the plan";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        let encoding = create_encoding(&tree);
        let encoded_bits = encode(text, &encoding);
        assert!(encoded_bits.is_some());
        let decoded_text = decode(&encoded_bits.unwrap(), &tree);
        assert!(decoded_text.is_some());
        assert_eq!(text, decoded_text.unwrap());
    }

    #[test]
    fn test11_encode_decode_single() {
        let text = "aaaaaa";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        let encoding = create_encoding(&tree);
        let encoded_bits = encode(text, &encoding);
        assert!(encoded_bits.is_some());
        let decoded_text = decode(&encoded_bits.unwrap(), &tree);
        assert!(decoded_text.is_some());
        assert_eq!(text, decoded_text.unwrap());
    }

    #[test]
    fn test12_encode_decode_empty() {
        let text = "";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        let encoding = create_encoding(&tree);
        let encoded_bits = encode(text, &encoding);
        assert!(encoded_bits.is_some());
        let decoded_text = decode(&encoded_bits.unwrap(), &tree);
        assert!(decoded_text.is_some());
        assert_eq!(text, decoded_text.unwrap());
    }

}


