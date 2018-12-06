use std::collections::*;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

const START_X: usize = 500;
const START_Y: usize = 500;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut points: Vec<(char, (usize, usize))> = Vec::new();

    // create points based on input

    let mut current_char = 'A';
    for a in input
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .map(|v| {
            (
                str::parse::<usize>(v[0].trim()).unwrap(),
                str::parse::<usize>(v[1].trim()).unwrap(),
            )
        }) {
        points.push((current_char, (START_Y + a.1, START_X + a.0)));
        current_char = (current_char as u8 + 1) as char;
    }

    // determine closest point + equidistance

    let mut ignore = HashSet::new();
    let mut distances: Vec<Vec<Option<(char, usize)>>> = vec![vec![None; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            for p in &points {
                if let Some((_, d)) = distances[y][x] {
                    if manhattan_distance(p.1, (y, x)) < d {
                        distances[y][x] = Some((p.0, manhattan_distance(p.1, (y, x))));

                        if ignore.contains(&(y, x)) {
                            ignore.remove(&(y, x));
                        }
                    } else if manhattan_distance(p.1, (y, x)) == d {
                        ignore.insert((y, x));
                    }
                } else {
                    distances[y][x] = Some((p.0, manhattan_distance(p.1, (y, x))));
                }
            }
        }
    }

    // set equidistant points to None

    for v in &ignore {
        distances[v.0][v.1] = None;
    }

    // calculate size of each region & number of points with total distance < 10000

    let mut region_sizes : HashMap<char, usize> = HashMap::new();
    let mut part2_region_size = 0;

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if distances[y][x].is_some() {
                *region_sizes
                    .entry(distances[y][x].unwrap().0)
                    .or_insert(0) += 1;
            }

            let total_distance : usize = points.iter().map(|p| manhattan_distance(p.1, (y, x))).sum();

            if total_distance < 10000 {
                part2_region_size += 1;
            }
        }
    }

    // find infinite regions

    let mut infinite_chars = HashSet::new();

    for x in 0..WIDTH {
        if distances[0][x].is_some() {
            infinite_chars.insert(distances[0][x].unwrap().0);
        }
        if distances[HEIGHT - 1][x].is_some() {
            infinite_chars.insert(distances[HEIGHT - 1][x].unwrap().0);
        }
    }
    for y in 0..HEIGHT {
        if distances[y][0].is_some() {
            infinite_chars.insert(distances[y][0].unwrap().0);
        }
        if distances[y][WIDTH - 1].is_some() {
            infinite_chars.insert(distances[y][WIDTH - 1].unwrap().0);
        }
    }

    // get largest region

    println!(
        "part 1: {}",
        region_sizes
            .iter()
            .filter(|v| !infinite_chars.contains(v.0))
            .max_by_key(|v| v.1)
            .unwrap()
            .1
    );

    // size of total_distance < 10000 region

    println!("part 2: {}", part2_region_size);
}

fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    ((p2.0 as isize - p1.0 as isize).abs() + (p2.1 as isize - p1.1 as isize).abs()) as usize
}
