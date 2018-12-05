use std::cmp::Ordering;
use std::collections::*;
use std::mem;
use std::str::FromStr;

fn main() {
    let original_input = include_str!("input.txt").trim();

    let mut replacements = Vec::new();

    // create all possible replacements

    for i in 65..=90 {
        let mut s_1 = String::new();

        s_1.push(i as u8 as char);
        s_1.push((i as u8 as char).to_ascii_lowercase());

        let mut s_2 = String::new();

        s_2.push((i as u8 as char).to_ascii_lowercase());
        s_2.push(i as u8 as char);

        replacements.push(s_1);
        replacements.push(s_2);
    }
    
    // part 2

    let mut lengths = Vec::new();

    for i in 0..26 {
        let mut input = original_input.to_string();

        input = input.replace((65 + (i as u8)) as char, "");
        input = input.replace((97 + (i as u8)) as char, "");

        lengths.push(fully_react(&input, &replacements).len());
    }

    println!(
        "part 1: {}",
        fully_react(&original_input, &replacements).len()
    );
    println!("part 2: {}", lengths.iter().min().unwrap());
}

fn fully_react(s: &str, replacements: &[String]) -> String {
    let mut input = s.to_string();
    let mut old_length = 0;

    loop {
        for r in replacements {
            input = input.replace(r, "");
        }

        if old_length == input.len() {
            break;
        } else {
            old_length = input.len();
        }
    }

    input
}
