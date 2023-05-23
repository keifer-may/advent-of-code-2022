use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;

fn num_elves(file_path: &Path) -> HashMap<u32, u32>{
    let mut map = HashMap::new();
    let mut elf_count: u32 = 0;
    let mut calories: u32 = 0;

    let mut file = File::open(file_path)
    .expect("Couldn't find file path");
    
    let mut contents = String::new();

    let lines = BufReader::new(file).lines();
    for line in lines {
        
        match line.unwrap().trim().parse::<u32>() {
                Ok(i) => {calories = calories + i},
                Err(..) => {
                    elf_count += 1;
                    map.insert(elf_count, calories);
                    calories = 0;
                },
            };
        }  
    map
}

fn main() {
    let input_path = Path::new("./src/input.txt");
    println!("{:?}", input_path);
    let my_elf_map = num_elves(&input_path);

    let most_cal_elf_num = my_elf_map.iter().max_by_key(|entry | entry.1).unwrap();
    println!("{:?}", most_cal_elf_num);

    let mut elf_vec: Vec<_> = my_elf_map.iter().collect();
    elf_vec.sort_by(|a,b| b.1.cmp(a.1));
    println!("{:?}", &elf_vec[0..3]);
}
