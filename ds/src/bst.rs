use std::fmt::Display;

pub struct BinarySearchTree<T: Ord + Display> {
    root: Option<Box<Node<T>>>,
    size: i32,
}

impl<T: Ord + Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn add(&mut self, value: T) {
        Self::add_(value, &mut self.root);
        self.size += 1;
    }

    fn add_(value: T, node: &mut Option<Box<Node<T>>>) {
        match node {
            None => *node = Some(Box::new(Node::new(value))),
            Some(ref mut contents) => {
                if value < contents.value {
                    Self::add_(value, &mut contents.left);
                } else {
                    Self::add_(value, &mut contents.right);
                }
            }
        }
    }

    pub fn iter(&self) -> InorderIterator<T> {
        InorderIterator::new(self)
    }

    pub fn inorder(&self) {
        Self::print_inorder_(&self.root);
    }

    fn print_inorder_(node: &Option<Box<Node<T>>>) {
        match node {
            None => (),
            Some(ref contents) => {
                Self::print_inorder_(&contents.left);
                println!("{}", contents.value);
                Self::print_inorder_(&contents.right);
            }
        }
    }
}

// impl<T : Ord + Display + Clone> Iterator for BinarySearchTree<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

struct Node<T: Ord + Display> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Display> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct InorderIterator<'a, T: Ord + Display> {
    stack: Vec<&'a Box<Node<T>>>
}

impl<'a, T: Ord + Display> InorderIterator<'a, T> {
    fn new(bst : &'a BinarySearchTree<T>) -> Self {
        let mut stack: Vec<&Box<Node<T>>> = Vec::new();
        let mut curr = & bst.root;
        while let Some(contents) = curr {
            stack.push(contents);
            curr = & contents.left;
        }
        InorderIterator { stack }
    }
}

impl<'a, T: Ord + Display> Iterator for InorderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let popped = self.stack.pop();
        if let Some(node) = popped {
            if let Some(ref right) = node.right {
                self.stack.push(right);
                let mut curr = & right.left;
                while let Some(ref contents) = curr {
                    self.stack.push(contents);
                    curr = & contents.left;
                }
            }
            return Some(& node.value);
        } 
        else {
            return None;
        }
    }
}

struct PreorderIterator {}

struct PostorderIterator {}
