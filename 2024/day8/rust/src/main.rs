use std::{
    char,
    collections::{HashMap, HashSet},
};

fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let input = get_input();
    println!("Rust solution");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!("---------------------------------------------")
}

fn parse_input(input: &str) -> HashMap<char, Vec<(i64, i64)>> {
    let mut result: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            if let Some(entry) = result.get_mut(&c) {
                entry.push((x.try_into().unwrap(), y.try_into().unwrap()));
            } else {
                result.insert(c, vec![(x.try_into().unwrap(), y.try_into().unwrap())]);
            }
        }
    }
    result
}

fn get_antinodes(
    max_x: i64,
    max_y: i64,
    antenna1: (i64, i64),
    antenna2: (i64, i64),
) -> [Option<(i64, i64)>; 2] {
    let mut result = [
        Some((
            antenna1.0 + (antenna1.0 - antenna2.0),
            antenna1.1 + (antenna1.1 - antenna2.1),
        )),
        Some((
            antenna2.0 + (antenna2.0 - antenna1.0),
            antenna2.1 + (antenna2.1 - antenna1.1),
        )),
    ];
    if result[0].unwrap().0 < 0
        || result[0].unwrap().1 < 0
        || result[0].unwrap().0 > max_x
        || result[0].unwrap().1 > max_y
    {
        result[0] = None;
    }
    if result[1].unwrap().0 < 0
        || result[1].unwrap().1 < 0
        || result[1].unwrap().0 > max_x
        || result[1].unwrap().1 > max_y
    {
        result[1] = None;
    }
    result
}

fn get_harmonic_antinodes(
    max_x: i64,
    max_y: i64,
    antenna1: (i64, i64),
    antenna2: (i64, i64),
) -> Vec<(i64, i64)> {
    let mut result = vec![];
    let (diff_x_1, diff_y_1) = (antenna1.0 - antenna2.0, antenna1.1 - antenna2.1);
    let (diff_x_2, diff_y_2) = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);
    let (mut temp_1, mut temp_2) = (antenna1, antenna2); //Include the antennas themselves

    while temp_1.0 >= 0 && temp_1.1 >= 0 && temp_1.0 <= max_x && temp_1.1 <= max_y {
        result.push(temp_1);
        temp_1 = (temp_1.0 + diff_x_1, temp_1.1 + diff_y_1);
    }
    while temp_2.0 >= 0 && temp_2.1 >= 0 && temp_2.0 <= max_x && temp_2.1 <= max_y {
        result.push(temp_2);
        temp_2 = (temp_2.0 + diff_x_2, temp_2.1 + diff_y_2);
    }
    result
}

fn part_one(input: &str) -> i64 {
    let data = parse_input(input);
    let max_x = input.lines().count() as i64 - 1;
    let max_y = input.lines().next().unwrap().chars().count() as i64 - 1;
    let mut set = HashSet::new();
    for (_, antennas) in data.iter() {
        for i in 0..(antennas.len() - 1) {
            for j in (i + 1)..antennas.len() {
                let antinodes = get_antinodes(max_x, max_y, antennas[i], antennas[j]);
                for antinode in antinodes.iter().flatten() {
                    set.insert(*antinode);
                }
            }
        }
    }

    set.len().try_into().unwrap()
}

fn part_two(input: &str) -> i64 {
    let data = parse_input(input);
    let max_x = input.lines().count() as i64 - 1;
    let max_y = input.lines().next().unwrap().chars().count() as i64 - 1;
    let mut set = HashSet::new();
    for (_, antennas) in data.iter() {
        for i in 0..(antennas.len() - 1) {
            for j in (i + 1)..antennas.len() {
                let antinodes = get_harmonic_antinodes(max_x, max_y, antennas[i], antennas[j]);
                for antinode in antinodes {
                    set.insert(antinode);
                }
            }
        }
    }

    set.len().try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_input() {
        let result = parse_input(SAMPLE_INPUT);
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&'0').unwrap().len(), 4);
        assert_eq!(result.get(&'A').unwrap().len(), 3);
    }

    #[test]
    fn test_get_antinodes() {
        let result = get_antinodes(11, 11, (5, 6), (8, 8));
        assert_eq!(result[0], Some((2, 4)));
        assert_eq!(result[1], Some((11, 10)));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE_INPUT), 14);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE_INPUT), 34);
    }
}
