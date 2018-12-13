use std::fmt;
use std::ops::{Add, AddAssign};

fn main() {
    let input = include_str!("input.txt");
    let mut carts = Vec::new();
    let mut grid = Vec::new();

    for (y, l) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in l.chars().enumerate() {
            match c {
                '>' | '<' | '^' | 'v' => {
                    let result = get_cart(x as isize, y as isize, c);
                    row.push(result.1);
                    carts.push(result.0);
                }
                '-' | '|' | '/' | '\\' | '+' | ' ' => row.push(c),
                _ => panic!("invalid character in input"),
            }
        }

        grid.push(row);
    }

    let grid = grid.into();
    let mut crash_has_happened = false;

    'main: loop {
        if carts.iter().filter(|v| !v.dead).count() == 1 {
            println!(
                "part 2: {}",
                carts.iter().filter(|c| !c.dead).nth(0).unwrap().pos
            );
            break 'main;
        }

        carts.sort_by_key(|v| v.pos.1);

        for i in 0..carts.len() {
            carts[i].tick(&grid);

            for j in 0..carts.len() {
                if i != j && carts[i].pos == carts[j].pos && !carts[i].dead && !carts[j].dead {
                    carts[i].dead = true;
                    carts[j].dead = true;

                    if !crash_has_happened {
                        println!("part 1: {}", carts[i].pos);
                        crash_has_happened = true;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Vector(isize, isize);

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_vel(&self) -> Vector {
        match self {
            Direction::Up => Vector(0, -1),
            Direction::Down => Vector(0, 1),
            Direction::Left => Vector(-1, 0),
            Direction::Right => Vector(1, 0),
        }
    }

    fn turn(&self, left: bool) -> Direction {
        if !left {
            match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            }
        }
    }
}

#[derive(PartialEq)]
struct Cart {
    pos: Vector,
    dir: Direction,
    counter: u64,
    dead: bool,
}

impl Cart {
    fn new(pos: Vector, dir: Direction) -> Cart {
        Cart {
            pos,
            dir,
            counter: 0,
            dead: false,
        }
    }

    fn tick(&mut self, grid: &Grid) {
        if !self.dead {
            self.pos += self.dir;

            match grid.get(self.pos) {
                '-' | '|' => {}
                '/' => {
                    self.dir = match self.dir {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Up,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Left,
                    }
                }
                '\\' => {
                    self.dir = match self.dir {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Right,
                    }
                }
                '+' => {
                    match self.counter {
                        0 => self.dir = self.dir.turn(true),
                        1 => {}
                        2 => self.dir = self.dir.turn(false),
                        _ => panic!("intersection failure"),
                    };

                    self.counter += 1;
                    self.counter %= 3;
                }
                _ => panic!(),
            }
        }
    }
}

struct Grid {
    inner: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, at: Vector) -> char {
        self.inner[at.1 as usize][at.0 as usize]
    }

    #[allow(dead_code)]
    fn print(&self, carts: &[Cart]) {
        for (y, row) in self.inner.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                match carts
                    .iter()
                    .filter(|c| !c.dead)
                    .find(|c| c.pos == Vector(x as isize, y as isize))
                {
                    Some(c) => print!(
                        "{}",
                        match c.dir {
                            Direction::Up => '^',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Right => '>',
                        }
                    ),
                    None => print!("{}", col),
                };
            }

            println!();
        }

        println!();
    }
}

impl From<Vec<Vec<char>>> for Grid {
    fn from(val: Vec<Vec<char>>) -> Self {
        Grid { inner: val }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Direction> for Vector {
    type Output = Vector;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.get_vel()
    }
}

impl AddAssign<Direction> for Vector {
    fn add_assign(&mut self, rhs: Direction) {
        let rhs = rhs.get_vel();
        *self = *self + rhs;
    }
}

fn get_cart(x: isize, y: isize, val: char) -> (Cart, char) {
    match val {
        '<' => (Cart::new(Vector(x, y), Direction::Left), '-'),
        '>' => (Cart::new(Vector(x, y), Direction::Right), '-'),
        '^' => (Cart::new(Vector(x, y), Direction::Up), '|'),
        'v' => (Cart::new(Vector(x, y), Direction::Down), '|'),
        _ => panic!("not a cart"),
    }
}
