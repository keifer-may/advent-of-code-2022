use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;

fn split_into_groups(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut temp = String::new();
    let mut our_string = input.clone();

    while our_string.len() > 0{
    for (index, item) in our_string.split_whitespace().enumerate() {
        let inner_temp = temp.clone();
        temp.push_str(item);

        // temp.push(',');

        if (index + 1) % 3 == 0 {
            result.push(temp.trim());
            our_string.replace(&temp, "");
            temp.clear();
            
            break;
            }
    }}


    result
}


fn main() {
    let mut letter_priority_map: HashMap<char, usize> = HashMap::new();

    let alphabet = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();

    println!("{}", alphabet);

    let mut alpha_chars_vec: Vec<_> = alphabet.chars().enumerate().collect();

    for entry in alpha_chars_vec.iter() {
        let character = entry.1;
        let priority = entry.0;
        let priority = u8::try_from(priority).expect("Can't convert priority");
        let priority = priority + 1;
        let priority = usize::try_from(priority).expect("Can't revert priority");
        letter_priority_map.insert(character, priority);
    }

    println!("My map of priorities: {:?}", letter_priority_map);

    let file_path = "./src/input.txt";

    let mut file = File::open(file_path)
    .expect("Couldn't find file path");

    // let lines = BufReader::new(file).lines();

    let mut priority: u32 = 0;

    // for line in lines {
    //     let total_rucksack = line.expect("Couldn't handle line").clone();
    //     let ruck_length = total_rucksack.len();
    //     let ruck_length_32 = u32::try_from(ruck_length).expect("Couldn't get rucksack legnth");
    //     let compartment_length = ruck_length_32 / 2;
    //     let compartment_length = usize::try_from(compartment_length).expect("Couldn't transform compartment length");
    //     let compart_one = &total_rucksack[..compartment_length];
    //     let compart_two = &total_rucksack[compartment_length..];
    //     // println!("Compart 1: {}", compart_one);
    //     // println!("Compart 2: {}", compart_two);
    //     for c in compart_one.chars() {
    //         if compart_two.contains(*&c) == true {
    //             let character:usize = *letter_priority_map.get(&c).expect("Couldn't get usize");
    //             let char_32: u32 = u32::try_from(character).expect("Couldn't convert usize to u32");
    //             priority += char_32;
    //             println!("The line's priority: {}\nThe running total: {}\nThe common character: {}", char_32, priority, c);
    //             break;
    //         }
    //     }
    // }




    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    println!("{}", contents);

    let all_groupings = split_into_groups(&contents);

    println!("{:?}", all_groupings);
    











    // for (index, line) in lines.enumerate() {
    //     let total_rucksack = line.expect("Couldn't handle line");
    //     println!("{}", total_rucksack);
    //     if line_one == "1" {
    //         line_one = &line.expect("nope");
    //         println!("line_one: {}", line_one);
    //         break;
    //     } else if line_two == "2" {
    //         line_two = &line.expect("nope");
    //         println!("line_two: {}", line_two);
    //         break;
    //     } else if line_three == "3" {
    //         line_three = &line.expect("nope");
    //         println!("line_three: {}", line_three);
    //         for c in line_one.chars() {
    //             if line_two.contains(c) == true && line_three.contains(c) == true {
    //                 let character:usize = *letter_priority_map.get(&c).expect("Couldn't get usize");
    //                 let char_32: u32 = u32::try_from(character).expect("Couldn't convert usize to u32");
    //                 priority += char_32;
    //                 println!("The group's priority: {}\nThe running total: {}\nThe common character: {}", char_32, priority, c);
    //                 let line_one = "1";
    //                 let line_two = "2";
    //                 let line_three = "3";
    //                 break;
    //             }
    //         }
    //     }
        
    // }




    println!("The overall priority score of the input file is: {}", priority);
}
