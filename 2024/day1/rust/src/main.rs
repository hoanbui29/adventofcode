use std::collections::HashMap;

fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_ids: Vec<i32> = vec![];
    let mut right_ids: Vec<i32> = vec![];
    for line in input.lines() {
        let mut parts = line.split("   ");
        left_ids.push(parts.next().unwrap().trim().parse().unwrap());
        right_ids.push(parts.next().unwrap().trim().parse().unwrap());
    }

    (left_ids, right_ids)
}

fn part_one() {
    let input = get_input();

    let (mut left_ids, mut right_ids) = parse_input(&input);
    left_ids.sort();
    right_ids.sort();

    let mut total_distance = 0;

    for i in 0..left_ids.len() {
        total_distance += (left_ids[i] - right_ids[i]).abs();
    }

    println!("Part one: {}", total_distance);
}

fn part_two() {
    let input = get_input();

    let (left_ids, right_ids) = parse_input(&input);
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for id in right_ids {
        let count = right_map.entry(id).or_insert(0);
        *count += 1;
    }
    let mut total = 0;

    for id in left_ids {
        let score = right_map.entry(id).or_insert(0);
        total += *score * id;
    }

    println!("Part two: {}", total);
}

fn main() {
    println!("Rust solutions");
    part_one();
    part_two();
    println!("----------------------------------------------------------------------")
}
