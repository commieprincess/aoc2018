use regex::Regex;

fn main() {
    let input = include_str!("input.txt").trim();
    let mut stars = Vec::new();

    let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();

    for c in re.captures_iter(&input) {
    	let star = Star::new((str::parse::<i64>(&c[1]).unwrap(), str::parse::<i64>(&c[2]).unwrap()), (str::parse::<i64>(&c[3]).unwrap(), str::parse::<i64>(&c[4]).unwrap()));
    	stars.push(star);
    }

    for i in 0.. {
    	if stars.iter().map(|v| v.has_neighbor(&stars)).all(|v| v) {
    		println!("found message at second {}", i);
    		println!();

    		let min_x = stars.iter().map(|v| v.pos.0).min().unwrap();
    		let max_x = stars.iter().map(|v| v.pos.0).max().unwrap();
    		let min_y = stars.iter().map(|v| v.pos.1).min().unwrap();
    		let max_y = stars.iter().map(|v| v.pos.1).max().unwrap();

    		for y in min_y..=max_y {
    			for x in min_x..=max_x {
    				let idx = stars.iter().map(|v| v.pos).position(|v| v == (x, y));

    				match idx {
    					Some(v) => {
    						print!("#");
    						stars.remove(v);
    					},
    					None => print!(" "),
    				}
    			}

    			println!();
    		}

    		println!();

    		break;
    	}

    	for s in &mut stars {
    		s.update();
    	}
    }
}

struct Star {
	pos: (i64, i64),
	vel: (i64, i64),
}

impl Star {
	fn new(pos: (i64, i64), vel: (i64, i64)) -> Star {
		Star {
			pos,
			vel,
		}
	}

	fn update(&mut self) {
		self.pos.0 += self.vel.0;
		self.pos.1 += self.vel.1;
	}

	fn has_neighbor(&self, stars: &[Star]) -> bool {
		for s in stars {
			if self.distance(&s) == 1 {
				return true;
			}
		}

		false
	}

	fn distance(&self, other: &Star) -> i64 {
		(self.pos.0 - other.pos.0 + self.pos.1 - other.pos.1).abs()
	}
}
