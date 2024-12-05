use std::collections::{HashMap, HashSet};

fn main() {
    let input = get_input();
    println!("Rust solution");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!("---------------------------------------------------")
}

fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut orders: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = Vec::new();
    let mut parse_orders = |data: &str| {
        let parts = data
            .split("|")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if let Some(orders) = orders.get_mut(&parts[0]) {
            orders.push(parts[1]);
        } else {
            orders.insert(parts[0], vec![parts[1]]);
        }
    };

    let mut parse_updates = |data: &str| {
        let parts = data
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        updates.push(parts);
    };

    let input_parts = input.split("\n\n").collect::<Vec<&str>>();

    //orders
    for line in input_parts[0].lines() {
        parse_orders(line);
    }
    //updates
    for line in input_parts[1].lines() {
        parse_updates(line);
    }

    (orders, updates)
}

fn is_correct(data: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..data.len() {
        if let Some(rule) = rules.get(&data[i]) {
            let set1: HashSet<_> = rule.iter().clone().collect();
            let set2: HashSet<_> = data[0..i].iter().clone().collect();
            if set1.intersection(&set2).count() > 0 {
                return false;
            }
        }
    }

    true
}

fn part_one(input: &str) -> i32 {
    let (orders, updates) = parse_input(input);
    let mut result = 0;
    for update in updates {
        if is_correct(&update, &orders) {
            result += update[update.len() / 2]
        }
    }

    result
}

fn part_two(input: &str) -> i32 {
    let (orders, updates) = parse_input(input);
    let mut result = 0;
    for mut update in updates {
        if is_correct(&update, &orders) {
            continue;
        }

        sort_updates(&mut update, &orders);
        result += update[update.len() / 2]
    }
    result
}

fn sort_updates(data: &mut [i32], rules: &HashMap<i32, Vec<i32>>) {
    data.sort_by(|a, b| {
        let default_value: Vec<i32> = Vec::new();
        let a_rules = rules.get(a).unwrap_or(&default_value);
        let b_rules = rules.get(b).unwrap_or(&default_value);
        if a_rules.contains(b) {
            std::cmp::Ordering::Less
        } else if b_rules.contains(a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_input() {
        let (orders, updates) = parse_input(SAMPLE_DATA);
        assert_eq!(orders.len(), 6);
        assert_eq!(updates.len(), 6);
    }

    #[test]
    fn test_is_correct() {
        let (orders, updates) = parse_input(SAMPLE_DATA);
        for update in updates {
            if is_correct(&update, &orders) {
                println!("{:?}", update);
            } else {
                println!("Not correct: {:?}", update);
            }
        }
    }

    #[test]
    fn test_sort_updates() {
        let (orders, updates) = parse_input(SAMPLE_DATA);
        for mut update in updates {
            if is_correct(&update, &orders) {
                continue;
            }
            sort_updates(&mut update, &orders);
            println!("{:?}", update);
        }
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE_DATA), 143);
    }
}
