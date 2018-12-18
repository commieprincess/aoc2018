use std::collections::*;

const GRID_SIZE: usize = 50;
const MINUTES: usize = 1_000_000_000;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

    let mut period_start = None;
    let mut scores = Vec::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            grid[y][x] = c;
        }
    }

    for _ in 0..MINUTES {
        let mut new_grid = grid.clone();
        let mut neighbors = HashMap::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                for dy in -1..=1isize {
                    if (dy == -1 && y == 0) || (dy == 1 && y == GRID_SIZE - 1) {
                        continue;
                    }

                    for dx in -1..=1isize {
                        if (dx == -1 && x == 0) || (dx == 1 && x == GRID_SIZE - 1) {
                            continue;
                        }

                        if !(dx == 0 && dy == 0) {
                            *neighbors
                                .entry(grid[(y as isize + dy) as usize][(x as isize + dx) as usize])
                                .or_insert(0) += 1;
                        }
                    }
                }

                match grid[y][x] {
                    '.' => {
                        if *neighbors.get(&'|').unwrap_or(&0) >= 3 {
                            new_grid[y][x] = '|';
                        }
                    }
                    '|' => {
                        if *neighbors.get(&'#').unwrap_or(&0) >= 3 {
                            new_grid[y][x] = '#';
                        }
                    }
                    '#' => {
                        if *neighbors.get(&'|').unwrap_or(&0) < 1
                            || *neighbors.get(&'#').unwrap_or(&0) < 1
                        {
                            new_grid[y][x] = '.';
                        }
                    }
                    _ => panic!(),
                };

                neighbors.clear();
            }
        }

        grid = new_grid;

        let wooded = grid
            .iter()
            .map(|v| v.iter().filter(|&tile| *tile == '|').count())
            .sum::<usize>();
        let yard = grid
            .iter()
            .map(|v| v.iter().filter(|&tile| *tile == '#').count())
            .sum::<usize>();

        match scores.iter().position(|&v| v == wooded * yard) {
            Some(p) => {
                if scores[p - 1] == scores[scores.len() - 1]
                    && scores[p - 2] == scores[scores.len() - 2]
                {
                    period_start = Some(p);
                    break;
                } else {
                    scores.push(wooded * yard);
                }
            }
            None => scores.push(wooded * yard),
        }
    }

    let period = &scores[period_start.unwrap()..scores.len() - 2];

    println!("part 1: {}", scores[9]);
    println!("part 2: {}", period[MINUTES % period.len()]);
}
