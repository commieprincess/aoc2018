fn main() {
    let input = include_str!("input.txt").trim();
    let serial_number = str::parse::<i64>(input).unwrap();

    let mut grid = vec![vec![0i64; 300]; 300];
    for x in 0..300 {
        for y in 0..300 {
            grid[x][y] = power_level((x, y), serial_number);
        }
    }

    let summed_area_table = summed_area_table(&grid);
    let mut top_part_1: Option<((usize, usize), i64)> = None;
    let mut top_part_2: Option<((usize, usize), i64, usize)> = None;

    for size in 1..300 {
        for x in 0..(300 - size) {
            for y in 0..(300 - size) {
                let total_power_level = summed_area_table[x][y]
                    + summed_area_table[x + size][y + size]
                    - summed_area_table[x + size][y]
                    - summed_area_table[x][y + size];

                if top_part_2.is_none() || total_power_level > top_part_2.unwrap().1 {
                    top_part_2 = Some(((x, y), total_power_level, size));
                }

                if size == 3 {
                    if top_part_1.is_none() || total_power_level > top_part_1.unwrap().1 {
                        top_part_1 = Some(((x, y), total_power_level));
                    }
                }
            }
        }
    }

    println!(
        "part 1: {},{}",
        (top_part_1.unwrap().0).0 + 1,
        (top_part_1.unwrap().0).1 + 1
    );

    println!(
        "part 2: {},{},{}",
        (top_part_2.unwrap().0).0 + 1,
        (top_part_2.unwrap().0).1 + 1,
        top_part_2.unwrap().2
    );
}


fn summed_area_table(grid: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut summed_area_table = vec![vec![0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            let mut value = grid[x][y];

            if y > 0 {
                value += summed_area_table[x][y - 1];
            }

            if x > 0 {
                value += summed_area_table[x - 1][y];
            }

            if x > 0 && y > 0 {
                value -= summed_area_table[x - 1][y - 1];
            }

            summed_area_table[x][y] = value;
        }
    }

    summed_area_table
}

fn power_level(coords: (usize, usize), serial_number: i64) -> i64 {
    let x = coords.0;
    let y = coords.1;

    let rack_id = x as i64 + 10;

    let mut power_level = ((rack_id * y as i64) + serial_number) * rack_id;;


    if power_level >= 100 {
        power_level /= 100;
        power_level %= 10;
    } else {
        power_level = 0;
    }

    power_level -= 5;

    power_level
}

// ORIGINAL ANSWER FOR PART 1

// for x in 0..297 {
//     for y in 0..297 {
//         let mut total_power_level = 0;

//         for i in 0..3 {
//             for j in 0..3 {
//                 total_power_level += grid[x + i][y + j];
//             }
//         }

//         if top_part_1.is_none() || total_power_level > top_part_1.unwrap().1 {
//             top_part_1 = Some(((x, y), total_power_level));
//         }
//     }
// }

// println!("part 1: ({}, {})", (top_part_1.unwrap().0).0, (top_part_1.unwrap().0).1);

// ================================================================================================

// ORIGINAL ANSWER FOR PART 2

// let mut top_part_2: Option<((usize, usize), i64, usize)> = None;

// for size in 1..300 {
//     for x in 0..(300 - size) {
//         for y in 0..(300 - size) {
//             let mut total_power_level = 0;

//             for i in 0..size {
//                 for j in 0..size {
//                     total_power_level += grid[x + i][y + j];
//                 }
//             }

//             if top_part_2.is_none() || total_power_level > top_part_2.unwrap().1 {
//                 top_part_2 = Some(((x, y), total_power_level, size));
//             }
//         }
//     }
// }
