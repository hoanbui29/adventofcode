use std::{
    char,
    collections::{hash_map::Entry, HashMap, HashSet},
};

fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let input = get_input();
    println!("Rust solution");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!("--------------------------------------------------------");
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Map {
    map: HashMap<(i32, i32), char>,
    steps: Vec<Position>,
    visited: HashSet<(i32, i32)>,
    start: (i32, i32),
}

impl Map {
    fn is_at_start(&self, pos: (i32, i32)) -> bool {
        self.start == (pos.0, pos.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn turn(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
            }
            Direction::Down => {
                self.direction = Direction::Left;
            }
            Direction::Left => {
                self.direction = Direction::Up;
            }
            Direction::Right => {
                self.direction = Direction::Down;
            }
        }
    }
}

fn mv(map: &mut Map, position: &mut Position) -> bool {
    let (x, y) = (position.x, position.y);
    let (x, y) = match position.direction {
        Direction::Up => (x - 1, y),
        Direction::Down => (x + 1, y),
        Direction::Left => (x, y - 1),
        Direction::Right => (x, y + 1),
    };

    if let Some(&c) = map.map.get(&(x, y)) {
        if c == '#' {
            position.turn(); // Meet obstacle, turn
            map.steps.push(position.clone());
            return false;
        }
    } else {
        return true; // End of the path
    }

    position.x = x;
    position.y = y;
    map.steps.push(position.clone());
    map.visited.insert((x, y));
    false
}

fn parse_input(input: &str) -> (Map, Position) {
    let mut matrix: HashMap<(i32, i32), char> = HashMap::new();
    let mut start: (i32, i32) = (0, 0);

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '^' {
                start = (x.try_into().unwrap(), y.try_into().unwrap());
            }
            matrix.insert((x.try_into().unwrap(), y.try_into().unwrap()), c);
        }
    }

    let (mut map, position) = (
        Map {
            map: matrix,
            steps: vec![],
            visited: HashSet::new(),
            start,
        },
        Position {
            x: start.0,
            y: start.1,
            direction: Direction::Up,
        },
    );

    map.steps.push(position.clone()); // Add starting point to the steps
    map.visited.insert(start); // Add starting point to the visited
    (map, position)
}

fn part_one(input: &str) -> i32 {
    let (mut map, mut start) = parse_input(input);
    let mut stop = false;
    while !stop {
        stop = mv(&mut map, &mut start);
    }

    map.visited.len().try_into().unwrap()
}

fn simulate_block(map: &mut Map, placed_blocks: &mut HashSet<(i32, i32)>, start: Position) -> bool {
    match start.direction {
        Direction::Up => {
            let point = (start.x - 1, start.y);
            if map.is_at_start(point) {
                return false;
            }
            if let Entry::Occupied(mut e) = map.map.entry(point) {
                if placed_blocks.contains(&point) {
                    return false;
                }
                e.insert('#');
                placed_blocks.insert(point);
            } else {
                return false;
            }
        }
        Direction::Down => {
            let point = (start.x + 1, start.y);
            if map.is_at_start(point) {
                return false;
            }

            if let Entry::Occupied(mut e) = map.map.entry(point) {
                if placed_blocks.contains(&point) {
                    return false;
                }
                e.insert('#');
                placed_blocks.insert(point);
            } else {
                return false;
            }
        }
        Direction::Left => {
            let point = (start.x, start.y - 1);
            if map.is_at_start(point) {
                return false;
            }

            if let Entry::Occupied(mut e) = map.map.entry(point) {
                if placed_blocks.contains(&point) {
                    return false;
                }
                e.insert('#');
                placed_blocks.insert(point);
            } else {
                return false;
            }
        }
        Direction::Right => {
            let point = (start.x, start.y + 1);
            if map.is_at_start(point) {
                return false;
            }

            if let Entry::Occupied(mut e) = map.map.entry(point) {
                if placed_blocks.contains(&point) {
                    return false;
                }
                e.insert('#');
                placed_blocks.insert(point);
            } else {
                return false;
            }
        }
    }
    let mut pointer = start.clone();
    let mut stop = false;
    let mut loop_steps: Vec<Position> = map.steps.to_vec();
    while !stop {
        loop_steps.push(pointer.clone());
        stop = mv(map, &mut pointer);
        if loop_steps.contains(&pointer) {
            break;
        }
    }
    !stop
}

fn part_two(input: &str) -> i32 {
    let (mut map, mut start) = parse_input(input);
    let mut stop = false;
    while !stop {
        stop = mv(&mut map, &mut start);
    }
    let mut total = 0;
    let mut placed_blocks = HashSet::new();
    for (idx, step) in map.steps.iter().enumerate() {
        let mut map = map.clone();
        map.steps = map.steps[..idx].to_vec();
        if simulate_block(&mut map, &mut placed_blocks, step.clone()) {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    use super::*;

    #[test]
    fn test_parse_input() {
        let (_, start) = parse_input(SAMPLE_INPUT);
        assert_eq!((start.x, start.y), (6, 4));
        assert_eq!(start.direction, Direction::Up);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE_INPUT), 41);
    }

    #[test]
    fn test_part_two() {
        let total = part_two(SAMPLE_INPUT);
        assert_eq!(total, 6);
    }
}
