use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// fn get_ranges(lines: <BufReader<std::fs::File>>) -> Vec<Vec<u16>, Vec<u16>> {
//     for line in lines {
//         let cloned_line = line.expect("Couldn't handle line").clone();
//         println!("{:?} {:?}", cloned_line, line);

//         let split_line: Vec<&str> = cloned_line.split(",").
//     }
// }

fn main() {
    let file_path = "./src/input.txt";

    let file = File::open(file_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();

    print_type_of(&lines);

    for line in lines {
        let cloned_line = line.expect("Couldn't handle line").clone();
        println!("{:?}", cloned_line);

        let split_line: Vec<&str> = cloned_line.split(",")
        .collect();

        println!("{:?}", split_line);

        let first_range = split_line[0];
        let second_range = split_line[1];

        

        for range in split_line {
            let split_nums: Vec<&str> = range.split("-").collect();
            let first_range = split_nums[0];
            let second_range = split_nums

            let start_num: u16 = split_nums[0].parse::<u16>().unwrap();
            let end_num: u16 = split_nums[1].parse::<u16>().unwrap();

            println!("Firstnum: {} Secondnum: {}", start_num, end_num);

        }
        
    }
}
