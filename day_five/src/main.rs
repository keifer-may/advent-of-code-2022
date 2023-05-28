use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;







#[derive(Debug, Clone)]
struct Instructions {
    num_objects: u16,
    from_vec: String,
    to_vec: String,
}

#[derive(Debug, Clone)]
struct GiftStacks {
    vec_1: Vec<char>,
    vec_2: Vec<char>,
    vec_3: Vec<char>,
    vec_4: Vec<char>,
    vec_5: Vec<char>,
    vec_6: Vec<char>,
    vec_7: Vec<char>,
    vec_8: Vec<char>,
    vec_9: Vec<char>,
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
            from_vec: ("vec_".to_owned() + from_vec).to_string(),
            to_vec:("vec_".to_owned() + to_vec).to_string()};

            let line_instructions: Instructions = line_instructions;

            list_instructions.push(line_instructions);
        }
    }

    return list_instructions;

}

fn main() {
    let mut vec_1: Vec<char> = ['W', 'D', 'G', 'B', 'H', 'R', 'V'].to_vec();
    let mut vec_2: Vec<char> = ['J', 'N', 'G', 'C', 'R', 'F'].to_vec();
    let mut vec_3: Vec<char> = ['L', 'S', 'F', 'H', 'D', 'N', 'J'].to_vec();
    let mut vec_4: Vec<char> = ['J', 'D', 'S', 'V'].to_vec();
    let mut vec_5: Vec<char> = ['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'].to_vec();
    let mut vec_6: Vec<char> = ['P', 'G', 'H', 'C', 'M'].to_vec();
    let mut vec_7: Vec<char> = ['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'].to_vec();
    let mut vec_8: Vec<char> = ['S', 'J', 'R'].to_vec();
    let mut vec_9: Vec<char> = ['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M'].to_vec();

    let mut starting_stacks = GiftStacks{vec_1:vec_1,
        vec_2:vec_2,
        vec_3:vec_3,
        vec_4:vec_4,
        vec_5:vec_5,
        vec_6:vec_6,
        vec_7:vec_7,
        vec_8:vec_8,
        vec_9:vec_9,};



    println!("Hello, world!");

    let input_path = "./src/input.txt";

    let list_instructions = get_line_instructions(input_path);

    for instructions in list_instructions{
        println!("{:?}", instructions);
    }

    println!("{:?}", starting_stacks);
}
