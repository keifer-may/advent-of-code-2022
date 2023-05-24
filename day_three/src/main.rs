use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;


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
        letter_priority_map.insert(character, priority);
    }

    
    println!("My map of priorities: {:?}", letter_priority_map);


    
}
