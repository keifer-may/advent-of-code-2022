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
    name: T,
    parent: Option<usize>,
    children: Vec<usize>,
    size: Option<u32>,
}

impl<T> Node<T>
where
    T: PartialEq
{
    fn new(idx: usize, name: T) -> Self {
        Self {
            idx,
            name,
            parent: None,
            children: vec![],
            size: None,
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq
{
    fn node(&mut self, name: T) -> usize {
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name));
        idx
    }

    fn set_working_dir(&mut self, name: T) -> usize {
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name));
        idx
    }

}

use std::io::BufRead;

fn main() {

   let mut my_test_tree = ArenaTree::default();
   
   my_test_tree.node("Cheese");
   my_test_tree.node("Chees");
   my_test_tree.node("5");
    println!("My tree: {:?}", my_test_tree);
    

    let input_path = "./src/input.txt";

    let file = File::open(input_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();

    // let mut file_read_reverse = String::from("");

    // for line in lines {
    //     let cloned_line = line.expect("Couldn't handle line").clone();
    //     if ! cloned_line.contains("cd ..") {
    //          file_read_reverse = cloned_line.to_string() + &file_read_reverse;
    //         println!("{}", cloned_line);
    //     }
    // }
    
    // println!("My reversed file: {}", file_read_reverse);

    let mut file_structure = ArenaTree::default();
    
    for line in lines {
        cloned_line = line.expect("Couldn't handle line").clone();
        
        
        if cloned_line.contains("..") {
            continue;
        } if cloned_line.contains("cd") {

        }

    }
}
