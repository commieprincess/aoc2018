use std::cmp::Ordering;
use std::collections::*;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut count_2 = 0;
    let mut count_3 = 0;

    for l in input.lines() {
        let mut freq: HashMap<char, usize> = HashMap::new();

        for c in l.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        if freq.values().any(|v| *v == 2) {
        	count_2 += 1;
        }

        if freq.values().any(|v| *v == 3) {
        	count_3 += 1;
        }
    }

    println!("part 1: {}", count_2 * count_3);

    'main_lines: for l in input.lines() {
        'nested_lines: for j in input.lines() {
            let mut count = 0;
            let mut common = String::new();

            if l != j {
                for (c, n) in l.chars().zip(j.chars()) {
                    if c != n {
                        count += 1;
                    } else {
                        common.push(c);
                    }

                    if count > 1 {
                        continue 'nested_lines;
                    }
                }
            }

            if count == 1 {
            	println!("part 2: {}", common);
                break 'main_lines;
            }
        }
    }
}
