use regex::Regex;

fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn part_one(input: &str) {
    let r = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut total = 0;
    for m in r.find_iter(&input).map(|m| m.as_str()) {
        total += m[4..m.len() - 1]
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .product::<i32>();
    }

    println!("Part one: {}", total);
}

fn part_two(input: &str) {
    let r = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)(?:n't)?\(\)").unwrap();

    let mut enabled = true;
    let mut total = 0;

    for (m, [x, y]) in r.captures_iter(input).map(|m| m.extract()) {
        match m {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                total += if enabled {
                    x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
                } else {
                    0
                };
            }
        }
    }

    println!("Part two: {}", total);
}

fn main() {
    let input = get_input();
    println!("Rust solution");
    part_one(&input);
    part_two(&input);
    println!("----------------------------------------");
}
