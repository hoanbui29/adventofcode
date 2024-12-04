use fancy_regex::Regex;

fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn part_one(input: &str) {
    let r = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut total = 0;
    for m in r.find_iter(&input).map(|m| m.unwrap().as_str()) {
        total += m[4..m.len() - 1]
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .product::<i32>();
    }

    println!("Part one: {}", total);
}

fn part_two(input: &str) {
    let enabled_groups = Regex::new(r"(?<=do\(\)|\A)[\s\S]*?(?=don\'t\(\)|\z)").unwrap();
    let mut total = 0;
    let enabled_groups = enabled_groups.find_iter(&input);
    for eg in enabled_groups {
        let r = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        for cap in r.find_iter(eg.unwrap().as_str()).map(|m| m.unwrap()) {
            let op = cap.as_str();
            total += op[4..op.len() - 1]
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .product::<i32>();
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
