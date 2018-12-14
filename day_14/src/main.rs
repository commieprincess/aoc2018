fn main() {
    let input: usize = include_str!("input.txt").trim().parse().unwrap();

    let input_digits = include_str!("input.txt")
        .chars()
        .map(|v| v.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut scoreboard = vec![3, 7];

    let mut elves = vec![0, 1];
    let mut first_round = true;

    loop {
        let mut two_added = false;

        let score = scoreboard[elves[0]] + scoreboard[elves[1]];
        if score >= 10 {
            scoreboard.push(1);
            two_added = true;
        }

        scoreboard.push(score % 10);

        if !first_round {
            elves[0] = (elves[0] + scoreboard[elves[0]] + 1) % scoreboard.len();
            elves[1] = (elves[1] + scoreboard[elves[1]] + 1) % scoreboard.len();
        } else {
            first_round = false;
        }

        if scoreboard.len() == input + 10 {
            print!("part 1: ");
            for i in 0..10 {
                print!("{}", scoreboard[scoreboard.len() - 10 + i]);
            }
            println!();
        }

        if scoreboard.len() > input_digits.len()
            && &scoreboard[scoreboard.len() - input_digits.len()..scoreboard.len()]
                == input_digits.as_slice()
        {
            println!("part 2: {}", scoreboard.len() - input_digits.len());
            break;
        } else if scoreboard.len() > input_digits.len()
            && two_added
            && &scoreboard[scoreboard.len() - input_digits.len() - 1..scoreboard.len() - 1]
                == input_digits.as_slice()
        {
            println!("part 2: {}", scoreboard.len() - input_digits.len() - 1);
            break;
        }
    }
}
