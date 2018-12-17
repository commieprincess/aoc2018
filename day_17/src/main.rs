use std::collections::*;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut can_reach: HashSet<(usize, usize)> = HashSet::new();

    let (mut grid, smallest_y) = parse_input(&input);

    flow_water(&mut grid, &mut can_reach, (500, 1));

    println!(
        "part 1: {}",
        can_reach.iter().filter(|v| v.1 >= smallest_y).count()
    );
    println!(
        "part 2: {}",
        grid.iter()
            .map(|v| v.iter().filter(|&&t| t == Tile::Water).count())
            .sum::<usize>()
    );

    // print_map(&grid, &can_reach);
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, usize) {
    let pattern = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)").unwrap();

    let mut grid = vec![vec![Tile::Sand; 5000]; 5000];

    let mut smallest_y = None;
    let mut largest_y = None;

    for l in input.lines() {
        let cap = pattern.captures(l).unwrap();

        let val_1 = (
            cap.get(1).unwrap().as_str(),
            str::parse::<usize>(cap.get(2).unwrap().as_str()).unwrap(),
        );
        let val_2 = (
            cap.get(3).unwrap().as_str(),
            str::parse::<usize>(cap.get(4).unwrap().as_str()).unwrap(),
            str::parse::<usize>(cap.get(5).unwrap().as_str()).unwrap(),
        );

        let y_min = match val_1.0 {
            "y" => val_1.1,
            "x" => val_2.1,
            _ => unreachable!(),
        };

        let y_max = match val_1.0 {
            "y" => val_1.1,
            "x" => val_2.2,
            _ => unreachable!(),
        };

        if largest_y.is_none() || y_max > largest_y.unwrap() {
            largest_y = Some(y_max);
        }

        if smallest_y.is_none() || y_min < smallest_y.unwrap() {
            smallest_y = Some(y_min);
        }

        for i in val_2.1..=val_2.2 {
            *get_at_mut(
                &mut grid,
                match val_1.0 {
                    "x" => (val_1.1, i),
                    "y" => (i, val_1.1),
                    _ => unreachable!(),
                },
            )
            .unwrap() = Tile::Clay;
        }
    }

    grid.truncate(largest_y.unwrap() + 1);

    (grid, smallest_y.unwrap())
}

#[allow(dead_code)]
fn print_map(grid: &[Vec<Tile>], can_reach: &HashSet<(usize, usize)>) {
    for y in 0..grid.len() {
        for x in 400..700 {
            print!(
                "{}",
                match get_at(&grid, (x, y)).unwrap() {
                    Tile::Clay => "#",
                    Tile::Sand => {
                        if can_reach.contains(&(x, y)) {
                            "|"
                        } else {
                            "."
                        }
                    }
                    Tile::Water => "~",
                }
            );
        }

        println!();
    }
}

fn flow_water(
    grid: &mut [Vec<Tile>],
    can_reach: &mut HashSet<(usize, usize)>,
    mut pos: (usize, usize),
) {
    let mut visited = Vec::new();

    loop {
        can_reach.insert(pos);
        visited.push(pos);

        if match get_at(&grid, (pos.0, pos.1 + 1)) {
            Some(tile) => *tile,
            None => return,
        } == Tile::Sand
        {
            pos = (pos.0, pos.1 + 1);
        } else {
            break;
        }
    }

    for &pos in visited.iter().rev() {
        let mut possible_water = HashSet::new();
        possible_water.insert(pos);

        if *get_at(grid, (pos.0, pos.1 + 1)).unwrap() != Tile::Sand {
            let mut wall_check_left = false;
            let mut wall_check_right = false;

            let mut left_pos = (pos.0 - 1, pos.1);

            while get_at(&grid, left_pos).is_some()
                && *get_at(&grid, left_pos).unwrap() == Tile::Sand
                && *get_at(&grid, (left_pos.0 + 1, left_pos.1 + 1)).unwrap() != Tile::Sand
            {
                possible_water.insert(left_pos);
                can_reach.insert(left_pos);

                if match get_at(&grid, (left_pos.0, left_pos.1 + 1)) {
                    Some(tile) => *tile,
                    None => break,
                } == Tile::Sand
                {
                    flow_water(grid, can_reach, (left_pos.0, left_pos.1 + 1));
                }

                left_pos = (left_pos.0 - 1, left_pos.1);
            }

            if get_at(&grid, left_pos)
                .map(|tile| *tile == Tile::Clay)
                .unwrap_or(false)
            {
                wall_check_left = true;
            }

            let mut right_pos = (pos.0 + 1, pos.1);

            while get_at(&grid, right_pos).is_some()
                && *get_at(&grid, right_pos).unwrap() == Tile::Sand
                && *get_at(&grid, (right_pos.0 - 1, right_pos.1 + 1)).unwrap() != Tile::Sand
            {
                possible_water.insert(right_pos);
                can_reach.insert(right_pos);

                if match get_at(&grid, (right_pos.0, right_pos.1 + 1)) {
                    Some(tile) => *tile,
                    None => break,
                } == Tile::Sand
                {
                    flow_water(grid, can_reach, (right_pos.0, right_pos.1 + 1));
                }

                right_pos = (right_pos.0 + 1, right_pos.1);
            }

            if get_at(&grid, right_pos)
                .map(|tile| *tile == Tile::Clay)
                .unwrap_or(false)
            {
                wall_check_right = true;
            }

            if wall_check_left && wall_check_right {
                for coord in &possible_water {
                    *get_at_mut(grid, *coord).unwrap() = Tile::Water;
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Clay,
    Sand,
    Water,
}

fn get_at(grid: &[Vec<Tile>], pos: (usize, usize)) -> Option<&Tile> {
    if pos.1 >= grid.len() || pos.0 >= grid[pos.1].len() {
        None
    } else {
        Some(&grid[pos.1][pos.0])
    }
}

fn get_at_mut(grid: &mut [Vec<Tile>], pos: (usize, usize)) -> Option<&mut Tile> {
    if pos.1 >= grid.len() || pos.0 >= grid[pos.1].len() {
        None
    } else {
        Some(&mut grid[pos.1][pos.0])
    }
}
