use std::io::BufReader;
use std::fs::File;
use std::fs;
// We are going to implement a tree structure here 
//and we'll be forced to parse the input file properly to populate it

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq + std::default::Default,
{
    arena: Vec<Node<T>>,
}

impl<T> Clone for ArenaTree<T>
where
    T: PartialEq + Clone + std::default::Default,
{
    fn clone(&self) -> Self {
        Self {
            arena: self.arena.clone(),
        }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    name: T,
    parent: Option<usize>,
    children: Vec<usize>,
    size: Option<u32>,
    dir: bool,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, name: T) -> Self {
        Self {
            idx,
            name,
            parent: None,
            children: vec![],
            size: None,
            dir: false,
        }
    }
}

impl<T> Clone for Node<T>
where
    T: PartialEq + Clone,
{
    fn clone(&self) -> Self {
        Self {
            idx: self.idx.clone(),
            name: self.name.clone(),
            parent: self.parent.clone(),
            children: self.children.clone(),
            size: self.size.clone(),
            dir: self.dir.clone(),
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq  + std::fmt::Debug + std::default::Default
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

    // fn set_working_dir(&mut self, name: T) -> usize {
    //     for node in &self.arena {
    //         if node.name == name {
    //             return node.idx;
    //         }
    //     }
    //     let idx = self.arena.len();
    //     self.arena.push(Node::new(idx, name));
    //     idx
    // }

    fn update_dir_size(&mut self) {
       for node in &self.arena{
            if node.dir == true {
                println!{"{:?}", node.size};
            }
       } 
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

    // let file = File::open(input_path).expect("Couldn't find file path");

    // let lines = BufReader::new(file).lines();

    // let mut file_read_reverse = String::from("");

    // for line in lines {
    //     let cloned_line = line.expect("Couldn't handle line").clone();
    //     if ! cloned_line.contains("cd ..") {
    //          file_read_reverse = cloned_line.to_string() + &file_read_reverse;
    //         println!("{}", cloned_line);
    //     }
    // }
    
    // println!("My reversed file: {}", file_read_reverse);

    // let mut file_structure = ArenaTree::default();
    
    let contents = fs::read_to_string(input_path).expect("Can't stringify contents");

    // println!("{}", contents);
    // if contents.contains('\n') {
    //     println!("New lines!");
    // }

    let list_commands: Vec<&str> = contents.trim_end().split("\n$ ").collect();

    // for command in list_commands {
    //     println!("{:?}", command);
    // }

    let mut file_structure: ArenaTree<String> = ArenaTree::default();

    let root_folder = file_structure.node("/".to_string());
    let root_index = file_structure.arena[root_folder].idx;
    let mut file_stack = vec![root_index];

    list_commands[2..].iter().for_each(|line| {
        let current_node_index = *file_stack.last().expect("No current node index");
        println!("Line: {:?}", line);
        if line.contains("cd") {
            // println!("{:?}", file_stack);
            let folder = line[3..].to_string();
            // println!("{:?}", folder);
            if folder == ".." {
                file_stack.pop();
                // println!("Pop! {:?}", file_stack);
            } else {
                let current_folder = file_structure.node(folder);
                let current_folder_index = file_structure.arena[current_folder].idx;
                file_structure.arena[current_folder].dir = true;
                file_structure.arena[current_folder].parent =
                    Some(current_node_index.try_into().unwrap());
                file_stack.push(current_folder_index);
                // println!("New stack: {:?}", file_stack);
            }
        }
        // println!("Node index: {:?}", current_node_index);
        if line.contains("ls\n") {
            // println!("LS {:?}", line);
            let split_files = line[3..].split('\n');
            // println!("{:?}", split_files);
            let mut folder_size: usize = 0;
            for file in split_files {
                let parts: Vec<&str> = file.split(' ').collect();
                // println!("{:?}", parts);
                let entity_name = parts[1].to_string();
                let is_folder = parts[0] == "dir";

                let entity_size = if is_folder {
                    0 as usize
                } else {
                    parts[0]
                        .parse::<usize>()
                        .expect("File size must be a number")
                };

                let node_for_new_entity = file_structure.node(entity_name);

                if !is_folder {
                    folder_size += entity_size;
                    file_structure.arena[node_for_new_entity].size =
                        Some(entity_size.try_into().unwrap());
                } else {
                    file_structure.arena[node_for_new_entity].dir = true;
                }
                file_structure.arena[node_for_new_entity].parent =
                    Some(current_node_index.try_into().unwrap());
                file_structure.arena[current_node_index]
                    .children
                    .push(node_for_new_entity);
            }

            file_structure.arena[current_node_index].size = Some(folder_size.try_into().unwrap());

            // println!("File tree: {:?}", file_structure);
        }
    });

    println!("{:?}", file_structure);

    file_structure.update_dir_size();

    // println!("{:?}", file_structure);
}
