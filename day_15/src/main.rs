use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::*;
use std::str::FromStr;
use std::thread::JoinHandle;

use std::sync::Arc;
use std::sync::Mutex;

type Location = (usize, usize);

fn main() {
    let input = include_str!("input.txt").trim();

    println!("part 1: {}", game(&input));
    println!("part 2: {}", game_p2(&input));
}

fn game(input: &str) -> u64 {
    let grid: Grid = str::parse(input).unwrap();
    let mut fighters = get_fighters(input, 3);

    let mut counter = 0;

    while game_round(&grid, &mut fighters) == GameResult::Next {
        counter += 1;
    }

    counter * fighters.iter().map(|f| f.hp()).sum::<u64>()
}

fn game_p2(input: &'static str) -> u64 {
    let grid: Arc<Grid> = Arc::new(str::parse(input).unwrap());

    let mut pool: Vec<JoinHandle<Option<(u64, u64)>>> = Vec::new();
    let counter = Arc::new(Mutex::new(Some(4)));

    for _ in 0..num_cpus::get() {
        let counter_handle = Arc::clone(&counter);
        let grid = Arc::clone(&grid);

        pool.push(std::thread::spawn(move || 'ap_loop: loop {
            let mut lock = counter_handle.lock().unwrap();

            let ap = (*lock)?;
            *lock.as_mut().unwrap() += 1;
            std::mem::drop(lock);

            let mut fighters = get_fighters(input, ap);

            for counter in 0.. {
                match game_round_p2(&grid, &mut fighters) {
                    GameResult::Failed => continue 'ap_loop,
                    GameResult::Next => continue,
                    GameResult::Done => {
                        *counter_handle.lock().unwrap() = None;
                        return Some((ap, counter * fighters.iter().map(|f| f.hp()).sum::<u64>()));
                    }
                };
            }
        }));
    }

    let mut values: Vec<(u64, u64)> = pool
        .into_iter()
        .map(|t| t.join().expect("could not join thread"))
        .filter_map(|v| v)
        .collect();

    values.sort_by_key(|v| v.1);

    values[0].1
}

fn get_fighters(input: &str, elf_ap: u64) -> Vec<Fighter> {
    let mut fighters = Vec::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'G' => fighters.push(Fighter::new((x, y), FighterType::Goblin, 3)),
                'E' => fighters.push(Fighter::new((x, y), FighterType::Elf, elf_ap)),
                _ => {}
            }
        }
    }

    fighters
}

#[derive(PartialEq)]
enum GameResult {
    Failed,
    Next,
    Done,
}

fn game_round_p2(grid: &Grid, fighters: &mut [Fighter]) -> GameResult {
    fighters.sort();

    for i in 0..fighters.len() {
        if !fighters[i].is_alive() {
            match fighters[i].fighter_type {
                FighterType::Elf => return GameResult::Failed,
                FighterType::Goblin => continue,
            };
        }

        if fighters[i].fighter_type == FighterType::Elf
            && fighters[i].find_enemies(fighters).is_empty()
        {
            return GameResult::Done;
        }

        do_turn(i, grid, fighters);
    }

    GameResult::Next
}

fn game_round(grid: &Grid, fighters: &mut [Fighter]) -> GameResult {
    fighters.sort();

    for i in 0..fighters.len() {
        if !fighters[i].is_alive() {
            continue;
        }

        if fighters[i].find_enemies(fighters).is_empty() {
            return GameResult::Done;
        }

        do_turn(i, grid, fighters);
    }

    GameResult::Next
}

fn do_turn(i: usize, grid: &Grid, fighters: &mut [Fighter]) {
    if fighters[i].find_attack_target(&fighters).is_none() {
        let reachable = fighters[i].reachable_squares(&grid, fighters);
        let mut potential_targets_in_order = vec![vec![]; 4];
        let mut potential_targets = Vec::new();

        for e in &fighters[i].find_enemies(&fighters) {
            let in_range = e.in_range(&grid, &fighters);

            for (i, s) in in_range
                .iter()
                .enumerate()
                .filter(|(_, s)| reachable.contains(s))
            {
                if reachable.contains(&s) {
                    potential_targets_in_order[i].push(s.clone());
                }
            }
        }

        for vec in &mut potential_targets_in_order {
            potential_targets.append(vec);
        }

        if !potential_targets.is_empty() {
            let closest_target_distance = potential_targets
                .iter()
                .filter_map(|v| {
                    shortest_path_between(fighters[i].pos, *v, &grid, &fighters)
                        .and_then(|v| Some(v.len()))
                })
                .min()
                .unwrap();

            potential_targets = potential_targets
                .iter()
                .filter_map(|t| {
                    shortest_path_between(fighters[i].pos, *t, &grid, &fighters)
                        .and_then(|v| Some(v.len()))
                        .map(|v| (t, v))
                })
                .filter(|t| t.1 == closest_target_distance)
                .map(|t| *t.0)
                .collect();

            let mut moves: Vec<(Location, usize)> = Vec::new();

            let reading_order = [
                (fighters[i].pos.0, fighters[i].pos.1 - 1),
                (fighters[i].pos.0 - 1, fighters[i].pos.1),
                (fighters[i].pos.0 + 1, fighters[i].pos.1),
                (fighters[i].pos.0, fighters[i].pos.1 + 1),
            ];

            for position in &reading_order {
                if grid.get(*position).is_empty()
                    && !fighters.iter().any(|f| f.pos == *position && f.is_alive())
                {
                    if let Some(v) =
                        shortest_path_between(*position, potential_targets[0], &grid, &fighters)
                    {
                        moves.push((*position, v.len()));
                    }
                }
            }

            if moves.is_empty() {
                return;
            }

            let min_len = moves.iter().min_by_key(|v| v.1).unwrap().1;
            fighters[i].pos = moves.iter().filter(|v| v.1 == min_len).nth(0).unwrap().0;
        }
    }

    fighters[i].attack(fighters);
}

fn manhattan_distance(first: Location, other: Location) -> usize {
    ((first.0 as isize - other.0 as isize).abs() + (first.1 as isize - other.1 as isize).abs())
        as usize
}

struct Fighter {
    hit_points: Cell<u64>,
    attack_power: u64,
    pos: Location,
    fighter_type: FighterType,
}

impl Fighter {
    fn new(pos: Location, fighter_type: FighterType, attack_power: u64) -> Fighter {
        Fighter {
            pos,
            fighter_type,
            hit_points: Cell::new(200),
            attack_power,
        }
    }

    fn find_attack_target<'a>(&self, fighters: &'a [Fighter]) -> Option<&'a Fighter> {
        let mut targets = self.find_enemies(fighters);
        targets.sort();
        let mut targets = targets
            .iter()
            .filter(|f| manhattan_distance(self.pos, f.pos) == 1)
            .cloned()
            .collect::<Vec<&Fighter>>();

        if targets.is_empty() {
            None
        } else {
            targets.sort_by_key(|f| f.hp());
            Some(targets[0])
        }
    }

    fn find_enemies<'a>(&self, fighters: &'a [Fighter]) -> Vec<&'a Fighter> {
        fighters
            .iter()
            .filter(|f| f.fighter_type != self.fighter_type && f.is_alive())
            .collect()
    }

    fn in_range(&self, grid: &Grid, fighters: &[Fighter]) -> Vec<Location> {
        let mut nodes = Vec::new();

        let reading_order = [
            (self.pos.0, self.pos.1 - 1),
            (self.pos.0 - 1, self.pos.1),
            (self.pos.0 + 1, self.pos.1),
            (self.pos.0, self.pos.1 + 1),
        ];

        for p in &reading_order {
            if grid.get(*p).is_empty() && !fighters.iter().any(|f| f.pos == *p && f.is_alive()) {
                nodes.push(*p);
            }
        }

        nodes
    }

    fn reachable_squares(&self, grid: &Grid, fighters: &[Fighter]) -> HashSet<Location> {
        let mut reachable = HashSet::new();

        let mut visited = HashSet::new();
        let mut to_consider: VecDeque<Location> = vec![
            (self.pos.0 - 1, self.pos.1),
            (self.pos.0 + 1, self.pos.1),
            (self.pos.0, self.pos.1 - 1),
            (self.pos.0, self.pos.1 + 1),
        ]
        .into();

        while !to_consider.is_empty() {
            let next = to_consider.pop_front().unwrap();
            visited.insert(next);

            if !reachable.contains(&next)
                && grid.get(next).is_empty()
                && !fighters.iter().any(|f| f.is_alive() && f.pos == next)
            {
                reachable.insert(next);

                let neighbors = vec![
                    (next.0, next.1 - 1),
                    (next.0 - 1, next.1),
                    (next.0 + 1, next.1),
                    (next.0, next.1 + 1),
                ];

                for p in &neighbors {
                    if !visited.contains(p) {
                        to_consider.push_back(*p);
                    }
                }
            }
        }

        reachable
    }

    fn take_damage(&self, dmg: u64) {
        let current_hp = self.hp();

        self.hit_points.set(current_hp.saturating_sub(dmg));
    }

    fn attack(&self, fighters: &[Fighter]) {
        if self.find_attack_target(&fighters).is_some() {
            self.find_attack_target(&fighters)
                .unwrap()
                .take_damage(self.attack_power);
        }
    }

    fn is_alive(&self) -> bool {
        self.hp() > 0
    }

    fn hp(&self) -> u64 {
        self.hit_points.get()
    }
}

impl PartialEq for Fighter {
    fn eq(&self, other: &Fighter) -> bool {
        self.hp() == other.hp()
            && self.attack_power == other.attack_power
            && self.pos == other.pos
            && self.fighter_type == other.fighter_type
    }
}

impl Eq for Fighter {}

fn shortest_path_between(
    self_pos: Location,
    target: Location,
    grid: &Grid,
    fighters: &[Fighter],
) -> Option<Vec<Location>> {
    let mut vertex_set: HashMap<Location, Option<(usize, Option<Location>)>> = HashMap::new();
    let mut visited: HashMap<Location, Option<(usize, Option<Location>)>> = HashMap::new();

    for y in 0..grid.inner.len() {
        for x in 0..grid.inner[y].len() {
            if grid.get((x, y)).is_empty()
                && !fighters.iter().any(|f| f.pos == (x, y) && f.is_alive())
            {
                vertex_set.insert((x, y), None);
            }
        }
    }

    vertex_set.insert(self_pos, Some((0, None)));

    let mut current = self_pos;

    loop {
        let neighbors = [
            (current.0, current.1 - 1),
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 + 1),
        ];

        let current_node_value = vertex_set[&current].unwrap();

        for p in neighbors.iter() {
            if let Some(neighbor_value) = vertex_set.get(p) {
                if neighbor_value.is_none() || neighbor_value.unwrap().0 > current_node_value.0 + 1
                {
                    vertex_set.insert(*p, Some((current_node_value.0 + 1, Some(current))));
                }
            }
        }

        visited.insert(current, vertex_set.remove(&current).unwrap());

        if current == target {
            break;
        }

        current = *vertex_set
            .iter()
            .filter(|(_, v)| v.is_some())
            .min_by_key(|(_, v)| v.unwrap().0)?
            .0;
    }

    let mut current = target;
    let mut path = Vec::new();

    while current != self_pos {
        path.push(visited[&current].unwrap().1.unwrap());
        current = visited[&current].unwrap().1.unwrap();
    }

    Some(path.iter().rev().cloned().collect())
}

impl PartialOrd for Fighter {
    fn partial_cmp(&self, other: &Fighter) -> Option<Ordering> {
        Some(match self.pos.1.cmp(&other.pos.1) {
            Ordering::Equal => self.pos.0.cmp(&other.pos.0),
            _ => self.pos.1.cmp(&other.pos.1),
        })
    }
}

impl Ord for Fighter {
    fn cmp(&self, other: &Fighter) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FighterType {
    Goblin,
    Elf,
}

#[derive(Clone)]
struct Grid {
    inner: Vec<Vec<NodeType>>,
}

impl Grid {
    fn get(&self, pos: Location) -> &NodeType {
        &self.inner[pos.1][pos.0]
    }

    fn print(&self, fighters: &[Fighter]) {
        for y in 0..self.inner.len() {
            for x in 0..self.inner[y].len() {
                if fighters.iter().any(|f| f.pos == (x, y) && f.is_alive()) {
                    print!(
                        "{}",
                        match fighters
                            .iter()
                            .find(|f| f.pos == (x, y))
                            .unwrap()
                            .fighter_type
                        {
                            FighterType::Goblin => 'G',
                            FighterType::Elf => 'E',
                        }
                    );
                } else {
                    print!(
                        "{}",
                        match self.get((x, y)) {
                            NodeType::Wall => '#',
                            NodeType::Empty => '.',
                        }
                    );
                }
            }

            for f in fighters.iter().filter(|f| f.pos.1 == y && f.is_alive()) {
                print!(
                    "\t{}({})",
                    match f.fighter_type {
                        FighterType::Goblin => 'G',
                        FighterType::Elf => 'E',
                    },
                    f.hp()
                );
            }

            println!();
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Grid, ()> {
        let mut grid = Vec::new();

        for l in s.lines().map(|l| l.trim()) {
            let mut row = Vec::new();

            for c in l.chars() {
                row.push(match c {
                    '#' => NodeType::Wall,
                    '.' | 'G' | 'E' => NodeType::Empty,
                    _ => panic!(),
                });
            }

            grid.push(row);
        }

        Ok(Grid { inner: grid })
    }
}

#[derive(Clone, Copy, PartialEq)]
enum NodeType {
    Wall,
    Empty,
}

impl NodeType {
    fn is_empty(self) -> bool {
        self == NodeType::Empty
    }
}
