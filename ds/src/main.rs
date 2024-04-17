mod bst;

use bst::BinarySearchTree;

fn main() {
    let mut tree = BinarySearchTree::<i32>::new();
    println!("Empty: {} Size: {}",tree.empty(), tree.size());
    tree.add(5);
    tree.add(9);
    tree.add(2);
    tree.add(17);
    tree.add(13);
    tree.add(8);
    println!("Empty: {} Size: {}",tree.empty(), tree.size());
    // let inorder = tree.inorder();
    // println!("Inorder: {:?}", inorder);
    for x in tree.iter() {
        println!("{}", x)
    }
    // let all = tree.iter().collect();
}