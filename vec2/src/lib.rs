#[derive(Debug, PartialEq, Clone)]
pub struct Node<T : PartialOrd> {
    data : T,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>,
}

pub struct Tree<T : PartialOrd> {
    root : Option<Box<Node<T>>>,
    length : usize,
}

impl<T : PartialOrd> Default for Tree<T> {
    fn default() -> Self { 
        Tree::new() 
    }
}

impl<T : PartialOrd> Tree<T> {
    pub fn new() -> Self {
        Self { root : None, length : 0 }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn add(&mut self, value : T) {
        Tree::add_recurse(&mut self.root, value);
        self.length += 1;
    }

    fn add_recurse(node : &mut Option<Box<Node<T>>>, value : T) {
        match node {
            Some(node_inner) => {
                if value < node_inner.data {
                    Tree::add_recurse(&mut node_inner.left, value);
                }
                else {
                    Tree::add_recurse(&mut node_inner.right, value);
                }
            }
            None => {
                *node = Some(Box::new(Node {data : value, left : None, right : None}));
            }
        }
    }

    // pub fn remove(&mut self, value : T) {
    //     Tree::remove_recurse(&mut self.root, value);
    // }

    // fn remove_recurse(node : &mut Option<Box<Node<T>>>, value : T) -> bool {
    //     match node {
    //         Some(node_inner) => {
    //             if value == node_inner.data {
    //                 if node_inner.left.is_none() && node_inner.right.is_none() {
    //                     *node = None;
    //                 } else if node_inner.left.is_none() || node_inner.right.is_none() {
    //                     if let Some(left) = node_inner.left {
    //                         // *node = 
    //                     }
                        
    //                 } else if node_inner.right.is_none() {
    //                     *node = node_inner.left;
    //                 }
    //                 return true;
    //             }
    //             if value < node_inner.data {
    //                 return Tree::remove_recurse(&mut node_inner.left, value);
    //             }
    //             return Tree::remove_recurse(&mut node_inner.right, value);
    //         }
    //         None => false
    //     }
    // }

    pub fn contains(&mut self, value : T) -> bool {
        Tree::contains_recurse(&self.root, value)
    }

    fn contains_recurse(node : &Option<Box<Node<T>>>, value : T) -> bool {
        match node {
            Some(node) => {
                if value == node.data {
                    return true;
                }
                if value < node.data {
                    return Tree::contains_recurse(&node.left, value);
                }
                Tree::contains_recurse(&node.right, value)
            }
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let t : Tree<u32> = Tree::new();
        assert_eq!(t.root, None);
        assert_eq!(t.len(), 0);
        assert!(t.is_empty());
    }

    #[test]
    fn test_add() {
        let mut t : Tree<u32> = Tree::new();
        t.add(5);
        t.add(3);
        t.add(7);
        t.add(2);
        t.add(4);
        t.add(6);
        t.add(8);
        assert_eq!(t.len(), 7);
        assert!(!t.is_empty());
        assert!(!t.contains(1));
        assert!(t.contains(2));
        assert!(t.contains(3));
        assert!(t.contains(4));
        assert!(t.contains(5));
        assert!(t.contains(6));
        assert!(t.contains(7));
        assert!(t.contains(8));
        assert!(!t.contains(9));
    }
}
