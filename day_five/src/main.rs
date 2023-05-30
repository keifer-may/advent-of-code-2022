use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Instructions {
    num_objects: u16,
    from_vec: String,
    to_vec: String,
}

impl Instructions{

    fn apply_instructions(&self, stacks: HashMap<String, Vec<char>>) -> HashMap<String, Vec<char>> {
        let get_from_vec = &self.from_vec;
        let get_to_vec = &self.to_vec;
        let get_num = self.num_objects;

        let mut map_stacks = stacks.clone();

        let mut from_vec = map_stacks.get(&*get_from_vec).expect("can't clone from vec").clone();
        let mut to_vec = map_stacks.get(&*get_to_vec).expect("can't clone to vec").clone();

        for _object in 0..get_num {
            let selected_object = from_vec.pop().unwrap();
            to_vec.push(selected_object);


        }

        map_stacks.insert(get_from_vec.to_string(), from_vec);
        map_stacks.insert(get_to_vec.to_string(), to_vec);

        return map_stacks;
    }

    fn new_application_instructions(&self, stacks: HashMap<String, Vec<char>>) -> HashMap<String, Vec<char>> {
        let get_from_vec = &self.from_vec;
        let get_to_vec = &self.to_vec;
        let get_num = self.num_objects;

        let mut map_stacks = stacks.clone();

        let mut from_vec = map_stacks.get(&*get_from_vec).expect("can't clone from vec").clone();
        let mut to_vec = map_stacks.get(&*get_to_vec).expect("can't clone to vec").clone();

        let range_to_select: usize = from_vec.len() - usize::try_from(get_num).expect("Can't convert selection");

        let slice_to_move = &from_vec[range_to_select..];

        for (iter, character) in slice_to_move.iter().enumerate() {
            to_vec.push(slice_to_move[iter]);
        }

        for object in 0..get_num{
            from_vec.pop();
        }

        map_stacks.insert(get_from_vec.to_string(), from_vec);
        map_stacks.insert(get_to_vec.to_string(), to_vec);

        return map_stacks;
    }

}

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

fn top_of_stacks (my_stacks: &mut HashMap<String, Vec<char>>) {
    for (stack, gift) in my_stacks.iter() {
        println!("The last gift is {} in stack {}", gift.last().unwrap().clone(), stack)
    }
}

fn main() {
    let vec_1: Vec<char> = ['W', 'D', 'G', 'B', 'H', 'R', 'V'].to_vec();
    let vec_2: Vec<char> = ['J', 'N', 'G', 'C', 'R', 'F'].to_vec();
    let vec_3: Vec<char> = ['L', 'S', 'F', 'H', 'D', 'N', 'J'].to_vec();
    let vec_4: Vec<char> = ['J', 'D', 'S', 'V'].to_vec();
    let vec_5: Vec<char> = ['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'].to_vec();
    let vec_6: Vec<char> = ['P', 'G', 'H', 'C', 'M'].to_vec();
    let vec_7: Vec<char> = ['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'].to_vec();
    let vec_8: Vec<char> = ['S', 'J', 'R'].to_vec();
    let vec_9: Vec<char> = ['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M'].to_vec();

    let mut starting_stacks: HashMap<String, Vec<char>> = HashMap::from([
        ("vec_1".to_string(), vec_1),
        ("vec_2".to_string(), vec_2),
        ("vec_3".to_string(), vec_3),
        ("vec_4".to_string(), vec_4),
        ("vec_5".to_string(), vec_5),
        ("vec_6".to_string(), vec_6),
        ("vec_7".to_string(), vec_7),
        ("vec_8".to_string(), vec_8),
        ("vec_9".to_string(), vec_9),
        ]);



    println!("Hello, world!");

    let input_path = "./src/input.txt";

    let list_instructions = get_line_instructions(input_path);

    for (iter, instructions) in list_instructions.iter().enumerate(){
        let new_stacks = starting_stacks;

        // println!("{:?}", instructions);

        starting_stacks = instructions.apply_instructions(new_stacks.clone());
    }

    println!("cloned starting stacks after operation{:?}", starting_stacks);
    top_of_stacks(&mut starting_stacks);





    let vec_1: Vec<char> = ['W', 'D', 'G', 'B', 'H', 'R', 'V'].to_vec();
    let vec_2: Vec<char> = ['J', 'N', 'G', 'C', 'R', 'F'].to_vec();
    let vec_3: Vec<char> = ['L', 'S', 'F', 'H', 'D', 'N', 'J'].to_vec();
    let vec_4: Vec<char> = ['J', 'D', 'S', 'V'].to_vec();
    let vec_5: Vec<char> = ['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'].to_vec();
    let vec_6: Vec<char> = ['P', 'G', 'H', 'C', 'M'].to_vec();
    let vec_7: Vec<char> = ['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'].to_vec();
    let vec_8: Vec<char> = ['S', 'J', 'R'].to_vec();
    let vec_9: Vec<char> = ['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M'].to_vec();

    let mut starting_stacks: HashMap<String, Vec<char>> = HashMap::from([
        ("vec_1".to_string(), vec_1),
        ("vec_2".to_string(), vec_2),
        ("vec_3".to_string(), vec_3),
        ("vec_4".to_string(), vec_4),
        ("vec_5".to_string(), vec_5),
        ("vec_6".to_string(), vec_6),
        ("vec_7".to_string(), vec_7),
        ("vec_8".to_string(), vec_8),
        ("vec_9".to_string(), vec_9),
        ]);



    println!("Hello, world!");

    let input_path = "./src/input.txt";

    let list_instructions = get_line_instructions(input_path);

    for (iter, instructions) in list_instructions.iter().enumerate(){
        let new_stacks = starting_stacks;

        // println!("{:?}", instructions);

        starting_stacks = instructions.new_application_instructions(new_stacks.clone());
    }

    println!("cloned starting stacks after operation{:?}", starting_stacks);
    top_of_stacks(&mut starting_stacks);
}
