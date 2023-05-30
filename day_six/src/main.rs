use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn main() {
    println!("Hello, world!");

    let input_path = "./src/input.txt";

    let file = File::open(input_path).unwrap();

    let lines = BufReader::new(file).lines();

    for line in lines {
        let clone_line = line.expect("Can't handle line").clone();
        let line_as_bytes = clone_line.chars();
        for (ind, _character) in line_as_bytes.enumerate() {
            let first_ind = ind;
            let second_ind: u16 =
                u16::try_from(first_ind).expect("Couldn't get rucksack legnth") + 14;
            let second_ind: usize = usize::try_from(second_ind).expect("couldn't get usize");
            let buffer = &clone_line[first_ind..second_ind];
            // println!("buffer: {:?}", buffer);

            match unique(buffer) {
                None => {
                    println!("{} is unique, message begins at {}", buffer, second_ind);
                    break;
                }
                Some((i, j, c)) => println!(
                    "{} is not unique\n\tfirst duplicate: \"{}\" (U+{:0>4X}) at indices {} and {}",
                    buffer, c, c as usize, i, j
                ),
            }
        }
    }
}
