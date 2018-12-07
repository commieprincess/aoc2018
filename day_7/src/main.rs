use std::collections::*;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut steps = HashMap::new();

    for l in input.lines() {
        let step = l.chars().nth(5).unwrap();
        let needed_to_start = l.chars().nth(36).unwrap();

        steps.entry(step).or_insert_with(Vec::new);
        steps
            .entry(needed_to_start)
            .or_insert_with(Vec::new)
            .push(step);
    }

    println!("part 1: {}", part_1(&steps));
    println!("{}", part_2(&steps, 5, 60));
}

fn part_1(steps: &HashMap<char, Vec<char>>) -> String {
    let mut order = Vec::new();

    loop {
        let mut possible_chars = steps.iter().filter(|v| {
    		let mut flag = true;

    		for c in v.1.iter() {
    			if !order.contains(&c) {
    				flag = false;
    			}
    		}

    		flag
    	} && !order.contains(&v.0)).collect::<Vec<(&char, &Vec<char>)>>();

        if possible_chars.is_empty() {
            break;
        }

        possible_chars.sort_by_key(|v| v.0);

        order.push(possible_chars[0].0);
    }

    order.iter().map(|v| *v).collect::<String>()
}

fn part_2(steps: &HashMap<char, Vec<char>>, workers: usize, base_time: usize) -> usize {
    let mut workers: Vec<Option<(char, usize)>> = vec![None; workers];

    let mut order = Vec::new();
    let mut in_progress = HashSet::new();

    let mut time = 0;

    loop {
        for w in &mut workers {
            let mut clear = false;

            match w {
                Some(ref mut val) => {
                    val.1 -= 1;

                    if val.1 == 0 {
                        order.push(val.0);
                        in_progress.remove(&val.0);
                        clear = true;
                    }
                }
                None => {}
            };

            if clear {
                *w = None;
            }
        }

        let mut possible_chars = steps.iter().filter(|v| {
    		let mut flag = true;

    		for c in v.1.iter() {
    			if !order.contains(&c) {
    				flag = false;
    			}
    		}

    		flag
    	} && !order.contains(&v.0) && !in_progress.contains(v.0)).collect::<Vec<(&char, &Vec<char>)>>();

        if possible_chars.is_empty() && in_progress.is_empty() {
            break;
        }

        possible_chars.sort_by_key(|v| v.0);

        for c in possible_chars.iter() {
            for w in &mut workers {
                if w.is_none() {
                    *w = Some((*c.0, base_time + ((*c.0 as u8) - ('A' as u8) + 1) as usize));
                    in_progress.insert(*c.0);
                    break;
                }
            }
        }

        time += 1;
    }

    time
}
