use std::collections::*;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut grid : Vec<Vec<Vec<usize>>> = vec![vec![vec![]; 1000]; 1000];

    let mut expected_dims : HashMap<usize, usize> = HashMap::new();

    for l in input.lines() {
    	let mut parts = l.split_whitespace().collect::<Vec<&str>>();

    	let claim = str::parse::<usize>(&parts[0][1..]).unwrap();
    	let mut start = &parts[2].split(',').map(|s| s.replace(":", "")).map(|s| str::parse::<usize>(&s).unwrap()).collect::<Vec<usize>>();
    	let mut dims = parts[3].split('x').map(|s| str::parse::<usize>(&s).unwrap()).collect::<Vec<usize>>();

    	expected_dims.insert(claim, dims[0] * dims[1]);

    	for x in start[0]..start[0] + dims[0] {
    		for y in start[1]..start[1] + dims[1] {
    			grid[y][x].push(claim);
    		}
    	}
    }

    let mut shared_claim_count = 0;
    let mut actual_dims : HashMap<usize, usize> = HashMap::new();

    for i in 0..1000 {
    	for j in 0..1000 {
    		if grid[j][i].len() == 1 {
    			*actual_dims.entry(grid[j][i][0] as usize).or_insert(0) += 1;
    		} else if grid[j][i].len() > 1 {
    			shared_claim_count += 1;
    		}
    	}
    }

    println!("part 1: {}", shared_claim_count);

    for (claim, unique_size) in actual_dims.iter() {
    	if expected_dims.get(claim).unwrap() == unique_size {
    		println!("part 2: {}", claim);
    		break;
    	}
    }
}
