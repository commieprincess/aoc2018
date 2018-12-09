use std::collections::*;

fn main() {
    let input = include_str!("input.txt").trim();
    let words = input.split_whitespace().collect::<Vec<&str>>();

    let players = str::parse(words[0]).unwrap();
    let marbles = str::parse(words[6]).unwrap();

    println!(
        "part 1: {}",
        MarbleCircle::top_score_from_params(marbles, players)
    );
    println!(
        "part 2: {}",
        MarbleCircle::top_score_from_params(marbles * 100, players)
    );
}

struct MarbleCircle {
    inner: VecDeque<usize>,
}

impl MarbleCircle {
    fn new() -> MarbleCircle {
        MarbleCircle {
            inner: (vec![0]).into(),
        }
    }

    fn top_score_from_params(marbles: usize, players: usize) -> usize {
        let mut circle = MarbleCircle::new();
        let mut scores = vec![0; players];

        for i in 1..=marbles {
            if i % 23 == 0 {
                scores[(i - 1) % players] += i;
                scores[(i - 1) % players] += circle.remove_marble();
            } else {
                circle.add_marble(i);
            }
        }

        *scores.iter().max().unwrap()
    }

    fn add_marble(&mut self, val: usize) {
        self.rotate_cw(2);
        self.inner.push_front(val);
    }

    fn remove_marble(&mut self) -> usize {
        self.rotate_ccw(7);
        self.inner.pop_front().unwrap()
    }

    pub fn rotate_cw(&mut self, val: usize) {
        for _ in 0..val {
            let v = self.inner.pop_front().unwrap();
            self.inner.push_back(v);
        }
    }

    pub fn rotate_ccw(&mut self, val: usize) {
        for _ in 0..val {
            let v = self.inner.pop_back().unwrap();
            self.inner.push_front(v);
        }
    }
}
