use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn calc_score(file_path: &Path) -> u32 {
    let mut score: u32 = 0;

    let winning_score: u32 = 6;
    let tie_score: u32 = 3;
    let losing_score: u32 = 0;

    let rock_score = 1;
    let paper_score = 2;
    let scissors_score = 3;

    let y_score = 3;
    let x_score = 0;
    let z_score = 6;

    let mut opponent_input: &str = "";
    let mut our_input: &str = "";

    let file = File::open(file_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();
    for line in lines {
        let handled_line = line.expect("Can't handle line").clone();
        let new_line = handled_line.trim();
        let mut iter = new_line.split_whitespace();
        opponent_input = iter.next().expect("Couldn't get opponent input");
        our_input = iter.next().expect("Couldn't get our input");

        if opponent_input == "A" && our_input == "Y" {
            score = score + y_score + rock_score;
        } else if opponent_input == "A" && our_input == "X" {
            score = score + x_score + scissors_score;
        } else if opponent_input == "A" && our_input == "Z" {
            score = score + z_score + paper_score;
        } else if opponent_input == "B" && our_input == "Y" {
            score = score + y_score + paper_score;
        } else if opponent_input == "B" && our_input == "X" {
            score = score + x_score + rock_score;
        } else if opponent_input == "B" && our_input == "Z" {
            score = score + z_score + scissors_score;
        } else if opponent_input == "C" && our_input == "Y" {
            score = score + y_score + scissors_score;
        } else if opponent_input == "C" && our_input == "X" {
            score = score + x_score + paper_score;
        } else if opponent_input == "C" && our_input == "Z" {
            score = score + z_score + rock_score;
        }
    }
    score
}

fn main() {
    let input_path = Path::new("./src/input.txt");
    println!("{:?}", input_path);

    let total_score = calc_score(input_path);

    println!("{}", total_score);
}
