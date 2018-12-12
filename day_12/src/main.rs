use std::collections::*;

const GENERATIONS : usize = 50_000_000_000;

fn main() {
    let input = include_str!("input.txt").trim();
    let mut rules: HashMap<&str, char> = HashMap::new();

    let lines = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();

    let mut state = "....".to_string();
    let zero = state.len() as i64;
    state.push_str(&lines[0][15..].to_string());
    state.push_str(".............................................................................................................................................................................................................................");

    for i in lines.iter().skip(2) {
        let parts = i.split("=>").map(|l| l.trim()).collect::<Vec<&str>>();

        rules.insert(parts[0], parts[1].chars().nth(0).unwrap());
    }

    let mut buf = Buffer::new();
    let mut last = 0;
    let mut last_number = 0;

    let mut constant = 0;

    for t in 0..GENERATIONS {
        let mut new_state = "..".to_string();

        for w in 2..state.len() - 2 {
            new_state.push(match rules.get(&state[(w - 2)..=(w + 2)]) {
                Some(v) => *v,
                None => state.chars().nth(w).unwrap(),
            });
        }

        new_state.push_str("..");

        let mut total: i64 = 0;
        for (i, c) in new_state.chars().enumerate() {
            if c == '#' {
                total += i as i64 - zero;
            }
        }

        if t == 19 {
        	println!("part 1: {}", total);
        }

        let diff = total - last;
        buf.add(diff);
        last = total;

        if buf.all_same() {
            constant = diff;
            last_number = t;
            break;
        }

        state = new_state;
    }

    println!("part 2: {}", value_at(last, last_number as i64, constant, GENERATIONS as i64 - 1));
}

fn value_at(last_val: i64, last_index: i64, constant: i64, at: i64) -> i64 {
	last_val + (at - last_index as i64) * constant
}

struct Buffer {
    inner: VecDeque<i64>,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            inner: VecDeque::new(),
        }
    }

    fn add(&mut self, new: i64) {
        if self.inner.len() == 100 {
            self.inner.pop_front();
        }

        self.inner.push_back(new);
    }

    fn all_same(&self) -> bool {
        if self.inner.len() == 100 {
            self.inner.iter().all(|v| *v == self.inner[0])
        } else {
            false
        }
    }
}
