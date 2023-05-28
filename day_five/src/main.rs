use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;







#[derive(Debug, Clone)]
struct Instructions {
    num_objects: u16,
    from_vec: String,
    to_vec: String,
}

// impl Instructions{

//     fn apply_instructions(list_of_instructions: Vec<Vec<char>>, &self) -> Vec<<Vec<char>> {

//     }





// }




fn get_line_instructions(file_path: &str) -> Vec<Instructions> {
    let mut list_instructions = Vec::new();

    let file = File::open(file_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();

    for line in lines {
        let cloned_line = line.expect("Couldn't handle line").clone();

        let mut split_line = cloned_line.split_whitespace();

        // let mut split_line = split_line.clone();

        let move_str = split_line.next().clone();

        if move_str == Some("move") {
            let count_objects = split_line.next().clone();
            let count_objects: u16 = count_objects.expect("can't handle number").parse::<u16>().unwrap();
            split_line.next();
            let from_vec = split_line.next().expect("can't get from vector");
            split_line.next();
            let to_vec = split_line.next().expect("can't get to vector");

            let line_instructions = Instructions{num_objects:count_objects,
            from_vec: from_vec.to_string(),
            to_vec:to_vec.to_string()};

            let line_instructions: Instructions = line_instructions;

            list_instructions.push(line_instructions);
        }
    }

    return list_instructions;

}

fn main() {
    println!("Hello, world!");

    let input_path = "./src/input.txt";

    let list_instructions = get_line_instructions(input_path);

    for instructions in list_instructions{
        println!("{:?}", instructions);
    }
}
