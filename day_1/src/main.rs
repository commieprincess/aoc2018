use std::cmp::Ordering;
use std::collections::*;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut counter = 0i64;
    let mut frequencies = Vec::new();

    let mut part_1_complete = false;

    'a: loop {
        for l in input.lines() {
            counter += str::parse::<i64>(l).unwrap();

            if frequencies.contains(&counter) {
                println!("part 2: {}", counter);
                break 'a;
            }

            frequencies.push(counter);
        }

        if !part_1_complete {
        	println!("part 1: {}", counter);
        	part_1_complete = true;
        }
    }
}
