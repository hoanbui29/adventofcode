fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        )
    }

    result
}

fn process_line(data: &Vec<i32>) -> i32 {
    if data[0] == data[1] {
        return 0;
    }
    let is_increase = data[0] < data[1];

    for i in 1..data.len() {
        if (is_increase && data[i] <= data[i - 1]) || (!is_increase && data[i] >= data[i - 1]) {
            return 0;
        }

        if (data[i] - data[i - 1]).abs() > 3 {
            return 0;
        }
    }

    1
}

fn process_line_tolerate_single_level(data: &Vec<i32>) -> i32 {
    let base_result = process_line(data);
    if base_result > 0 {
        return base_result;
    }

    for i in 0..data.len() {
        let new_data: Vec<&i32> = data[0..i].iter().chain(data[i + 1..].iter()).collect();
        let new_result = process_line(&new_data.iter().map(|x| **x).collect());
        if new_result > 0 {
            return new_result;
        }
    }

    0
}

fn part_one(input: &str) {
    let data = parse_input(input);
    let mut total = 0;
    for i in 0..data.len() {
        total += process_line(&data[i]);
    }
    println!("Part one: {}", total);
}

fn part_two(input: &str) {
    let data = parse_input(input);
    let mut total = 0;
    for i in 0..data.len() {
        total += process_line_tolerate_single_level(&data[i]);
    }

    println!("Part two: {}", total);
}

fn main() {
    println!("Rust solutions");
    let input = get_input();
    part_one(&input);
    part_two(&input);
    println!("----------------------------------------------------------------------")
}
