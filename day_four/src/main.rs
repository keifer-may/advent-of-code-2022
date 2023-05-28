use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[derive(Debug)]
struct ElfPairRanges {
    elf_one: Range<u16>,
    elf_two: Range<u16>,
}

impl ElfPairRanges{

    fn check_ranges_fully_overlap(&self) -> bool {
        let mut contained_one: bool = true;

        let range_one = self.elf_one.clone();
        let range_two = self.elf_two.clone();

        // println!("{:?} {:?}", range_one, range_two);

        for num in range_one {
            if !self.elf_two.contains(&num) {
                contained_one = false;
                break;
            }
        }

        let mut contained_two: bool = true;

        for num in range_two {
            if !self.elf_one.contains(&num) {
                contained_two = false;
                break;
            }
        }
        
        if contained_one == true || contained_two == true {
            return true;
        } else {
            return false;
        }
    }


    fn check_ranges_duplicate_work(&self) -> bool {
        let mut contained_one: bool = false;

        let range_one = self.elf_one.clone();
        let range_two = self.elf_two.clone();

        for num in range_one {
            if self.elf_two.contains(&num) {
                contained_one = true;
                return true;
            }
        }

        let mut contained_two: bool = false;

        for num in range_two {
            if self.elf_one.contains(&num) {
                contained_two = true;
                return true;
            }
        }
        
        return false;
    }

}




fn create_ranges<U16>(file_path: &str) -> Vec<ElfPairRanges> {
    let file = File::open(file_path).expect("Couldn't find file path");

    let lines = BufReader::new(file).lines();

    let mut my_ranges = Vec::new();

    for line in lines {
        let cloned_line = line.expect("Couldn't handle line").clone();
        // println!("{:?}", cloned_line);

        let split_line: Vec<&str> = cloned_line.split(",")
        .collect();


        // println!("{:?}", split_line);

        let first_range = split_line[0];
        let second_range = split_line[1];

        let split_first_range: Vec<&str> = first_range.split("-").collect();
        let split_second_range: Vec<&str> = second_range.split("-").collect();

        let first_range_start_num: u16 = split_first_range[0].parse::<u16>().unwrap();
        let first_range_end_num: u16 = split_first_range[1].parse::<u16>().unwrap();

        let second_range_start_num: u16 = split_second_range[0].parse::<u16>().unwrap();
        let second_range_end_num: u16 = split_second_range[1].parse::<u16>().unwrap();

        my_ranges.push(ElfPairRanges{elf_one:(first_range_start_num..first_range_end_num+1), elf_two: (second_range_start_num..second_range_end_num+1)});
    }

    my_ranges
}



fn main() {
    let file_path = "./src/input.txt";

    let my_ranges = create_ranges::<u16>(file_path);

    let mut count_fully_contained_ranges = 0;

    println!("{:?}", my_ranges);

    for i in my_ranges.iter() {
        let my_checked_ranges = i.check_ranges_fully_overlap();

        // println!("{:?}", my_checked_ranges);

        if my_checked_ranges == true {
            count_fully_contained_ranges += 1;
        }
    }

    println!("\n\nThe total number of pairs that have one range fully contained in the other is: {}", count_fully_contained_ranges);

    let mut count_duplicated_ranges = 0;

    for i in my_ranges.iter() {
        let my_checked_ranges = i.check_ranges_duplicate_work();

        // println!("{:?}", my_checked_ranges);

        if my_checked_ranges == true {
            count_duplicated_ranges += 1;
        }
    }

    println!("\n\nThe total number of assignments that share ranges is: {}", count_duplicated_ranges);

}
