use std::io::BufReader;
use std::fs::File;
// We are going to implement a tree structure here 
//and we'll be forced to parse the input file properly to populate it

#[derive(Debug, Default)]
struct ArenaTree<T> 
where
    T: PartialEq
{
    arena: Vec<Node<T>>,
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq
{
    fn node(&mut self, val: T, name: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name));
        idx
    }
}

use std::io::BufRead;

fn main() {

   let mut my_test_tree = ArenaTree::default();
   
   my_test_tree.node("Cheese", "My favorite");
   my_test_tree.node("Chees", "My favorite misspeeeled");
   my_test_tree.node("5", "My fav num");
    println!("My tree: {:?}", my_test_tree);
    

    let input_path = "./src/input.txt";

    let file = File::open(input_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();

    let mut file_read_reverse = String::from("");

    for line in lines {
        let cloned_line = line.expect("Couldn't handle line").clone();
        if ! cloned_line.contains("cd ..") {
             file_read_reverse = cloned_line.to_string() + &file_read_reverse;
            println!("{}", cloned_line);
        }
    }
    
    println!("My reversed file: {}", file_read_reverse);

}
