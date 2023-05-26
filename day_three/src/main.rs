use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut letter_priority_map: HashMap<char, usize> = HashMap::new();

    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    // println!("{}", alphabet);

    let alpha_chars_vec: Vec<_> = alphabet.chars().enumerate().collect();

    for entry in alpha_chars_vec.iter() {
        let character = entry.1;
        let priority = entry.0;
        let priority = u8::try_from(priority).expect("Can't convert priority");
        let priority = priority + 1;
        let priority = usize::try_from(priority).expect("Can't revert priority");
        letter_priority_map.insert(character, priority);
    }

    // println!("My map of priorities: {:?}", letter_priority_map);

    let file_path = "./src/input.txt";

    let file = File::open(file_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();

    let mut priority: u32 = 0;

    for line in lines {
        let total_rucksack = line.expect("Couldn't handle line").clone();
        let ruck_length = total_rucksack.len();
        let ruck_length_32 = u32::try_from(ruck_length).expect("Couldn't get rucksack legnth");
        let compartment_length = ruck_length_32 / 2;
        let compartment_length =
            usize::try_from(compartment_length).expect("Couldn't transform compartment length");
        let compart_one = &total_rucksack[..compartment_length];
        let compart_two = &total_rucksack[compartment_length..];
        // println!("Compart 1: {}", compart_one);
        // println!("Compart 2: {}", compart_two);
        for c in compart_one.chars() {
            if compart_two.contains(*&c) == true {
                let character: usize = *letter_priority_map.get(&c).expect("Couldn't get usize");
                let char_32: u32 = u32::try_from(character).expect("Couldn't convert usize to u32");
                priority += char_32;
                // println!("The line's priority: {}\nThe running total: {}\nThe common character: {}", char_32, priority, c);
                break;
            }
        }
    }

    println!(
        "The overall priority score of the input file for problem 1 is: {}",
        priority
    );

    // Begin problem 2, resetting our tracking priority value and file handling
    let mut priority: u32 = 0;

    let file_path = "./src/input.txt";

    let mut file = File::open(file_path).expect("Couldn't find file path");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // println!("{}", contents);

    let lines: Vec<&str> = contents.split('\n').collect();
    let matches = lines.windows(3).step_by(3);

    // println!("{:?}", matches);

    for group in matches {
        // println!("{:?}", group);
        // println!("{:?}", group[0]);
        let ruck_one = group[0];
        let ruck_two = group[1];
        let ruck_three = group[2];

        for c in ruck_one.chars() {
            if ruck_two.contains(c) && ruck_three.contains(c) {
                let character: usize = *letter_priority_map.get(&c).expect("Couldn't get usize");
                let char_32: u32 = u32::try_from(character).expect("Couldn't convert usize to u32");
                priority += char_32;
                // println!("The line's priority: {}\nThe running total: {}\nThe common character: {}", char_32, priority, c);
                break;
            }
        }
    }

    println!(
        "The overall priority score of the input file for problem 2 is: {}",
        priority
    );
}
