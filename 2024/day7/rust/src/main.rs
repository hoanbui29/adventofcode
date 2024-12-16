#[derive(Clone, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let input = read_input();
    println!("Rust solution");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!("---------------------------------------------")
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut test_values: Vec<(usize, Vec<usize>)> = vec![];
    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        test_values.push((
            parts[0].parse().unwrap(),
            parts[1].split(" ").map(|x| x.parse().unwrap()).collect(),
        ));
    }
    test_values
}

fn calc_operations(numbers: &[usize], operators: &[Operator]) -> usize {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        let op = &operators[i - 1];
        result = match op {
            Operator::Add => result + numbers[i],
            Operator::Mul => result * numbers[i],
            Operator::Concat => {
                let mut digits = vec![];
                let mut n = numbers[i];
                while n > 0 {
                    digits.push(n % 10);
                    n /= 10;
                }
                digits.reverse();
                for d in digits {
                    result = result * 10 + d;
                }
                result
            }
        };
    }
    result
}

fn get_operator_variations_bit(total: usize) -> Vec<Vec<Operator>> {
    let mut result = vec![];
    let total_variations = 1 << (total);
    for num in 0..total_variations {
        let mut operators = vec![];
        for i in 0..total {
            let op = if num & (1 << i) == 0 {
                Operator::Add
            } else {
                Operator::Mul
            };
            operators.push(op);
        }
        result.push(operators);
    }
    result
}

fn get_operator_variations(total: usize, base: usize) -> Vec<Vec<Operator>> {
    let mut result = vec![];
    let total_variations = base.pow(total as u32);
    for num in 0..total_variations {
        let mut operators = vec![];
        let mut n = num;
        while n > 0 {
            match n % base {
                0 => operators.push(Operator::Add),
                1 => operators.push(Operator::Mul),
                2 => operators.push(Operator::Concat),
                _ => panic!("Invalid operator"),
            }
            n /= base;
        }
        if operators.len() < total {
            operators.extend(std::iter::repeat(Operator::Add).take(total - operators.len()));
        }
        operators.reverse();
        result.push(operators);
    }
    result
}

fn part_one(input: &str) -> usize {
    let mut result = 0;
    let data = parse_input(input);
    for sample in data {
        for operator in get_operator_variations_bit(sample.1.len() - 1) {
            if calc_operations(&sample.1, &operator) == sample.0 {
                result += sample.0;
                break;
            }
        }
    }
    result
}

fn part_two(input: &str) -> usize {
    let mut result = 0;
    let data = parse_input(input);
    for sample in data {
        for operator in get_operator_variations(sample.1.len() - 1, 3) {
            if calc_operations(&sample.1, &operator) == sample.0 {
                result += sample.0;
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        let expected = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        assert_eq!(parse_input(SAMPLE_INPUT), expected);
    }

    #[test]
    fn test_calc_operations() {
        let numbers = vec![1, 2, 3, 4];
        let operators = vec![Operator::Add, Operator::Mul, Operator::Add];
        assert_eq!(calc_operations(&numbers, &operators), 13); //Predecence rule is ignored
    }

    #[test]
    fn test_get_operator_variations_bit() {
        let operators = get_operator_variations_bit(2);
        assert_eq!(operators.len(), 4);
        println!("{:?}", &operators);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE_INPUT), 3749);
    }

    #[test]
    fn test_get_operator_variations() {
        let operators = get_operator_variations(3, 3);
        assert_eq!(operators.len(), 27);
        println!("{:?}", &operators);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE_INPUT), 11387);
    }

    #[test]
    fn test_operations() {
        let numbers = vec![123, 45, 3, 678];
        let operators = vec![Operator::Concat, Operator::Add, Operator::Concat];
        assert_eq!(calc_operations(&numbers, &operators), 12348678); //Predecence rule is ignored
    }
}
