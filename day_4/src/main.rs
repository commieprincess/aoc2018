use std::cmp::Ordering;
use std::collections::*;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut minutes: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut days: HashMap<Date, (u32, Vec<bool>)> = HashMap::new();

    let mut on_shift = 0;
    let mut start_sleep: Option<u32> = None;

    let mut lines = input
        .lines()
        .map(|s| {
            (
                Timestamp::from_string(&s[1..17]),
                match s[19..].trim() {
                    "falls asleep" => Instruction::Sleep,
                    "wakes up" => Instruction::Wake,
                    _ => Instruction::StartShift(
                        str::parse(
                            &(&s[19..].trim().split_whitespace().collect::<Vec<&str>>()[1])[1..],
                        ).unwrap(),
                    ),
                },
            )
        }).collect::<Vec<(Timestamp, Instruction)>>();

    lines.sort_by_key(|v| v.0.clone());

    for l in &lines {
        match l.1 {
            Instruction::StartShift(g) => on_shift = g,
            Instruction::Sleep => start_sleep = Some(l.0.minute),
            Instruction::Wake => {
                for i in start_sleep.unwrap()..=l.0.minute {
                    days.entry(l.0.date.clone())
                        .or_insert((on_shift, vec![false; 60]))
                        .1[i as usize] = true;
                    minutes.entry(on_shift).or_insert(vec![0; 60])[i as usize] += 1;
                }

                start_sleep = None;
            }
        }
    }

    let longest_guard = minutes
        .iter()
        .map(|(g, v)| (g, v.iter().sum::<u32>()))
        .max_by_key(|k| k.1)
        .unwrap()
        .0;
    let most_slept_minute = days
        .iter()
        .filter(|s| (s.1).0 == *longest_guard)
        .fold(vec![0u32; 60], |mut acc, v| {
            for i in 0..60 {
                acc[i] += if (v.1).1[i] { 1 } else { 0 }
            }

            acc
        }).iter()
        .enumerate()
        .max_by_key(|v| v.1)
        .unwrap()
        .0 as u32
        - 1;

    println!("part 1: {}", longest_guard * most_slept_minute);

    let mut max_minute = 0;
    let mut data = (0, 0);

    for (g, m) in minutes.iter() {
        for (i, minute) in m.iter().enumerate() {
            if *minute > max_minute {
                data = (*g, i);
                max_minute = *minute;
            }
        }
    }

    println!("part 2: {}", data.0 * data.1 as u32);
}

#[derive(Debug)]
enum Instruction {
    StartShift(u32),
    Sleep,
    Wake,
}

#[derive(PartialEq, Eq, Ord, Clone, Debug, Hash)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(PartialEq, Eq, Ord, Clone, Debug)]
struct Timestamp {
    date: Date,
    hour: u32,
    minute: u32,
}

impl Timestamp {
    fn from_string(s: &str) -> Timestamp {
        Timestamp {
            date: Date {
                year: str::parse(&s[0..4]).unwrap(),
                month: str::parse(&s[5..=6]).unwrap(),
                day: str::parse(&s[8..=9]).unwrap(),
            },
            hour: str::parse(&s[11..=12]).unwrap(),
            minute: str::parse(&s[14..=15]).unwrap(),
        }
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        if self.date != other.date {
            Some(self.date.cmp(&other.date))
        } else if self.hour != other.hour {
            Some(self.hour.cmp(&other.hour))
        } else if self.minute != other.minute {
            Some(self.minute.cmp(&other.minute))
        } else {
            Some(Ordering::Equal)
        }
    }
}


impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        if self.year != other.year {
            Some(self.year.cmp(&other.year))
        } else if self.month != other.month {
            Some(self.month.cmp(&other.month))
        } else if self.day != other.day {
            Some(self.day.cmp(&other.day))
        } else {
            Some(Ordering::Equal)
        }
    }
}
