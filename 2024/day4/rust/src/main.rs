fn get_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut line_result = vec![];
        for c in line.chars() {
            line_result.push(c);
        }
        result.push(line_result);
    }
    result
}

fn get_xmas_arount(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    let mut total = 0;
    if y > 2 && x > 2 {
        if matrix[y - 1][x - 1] == 'M' && matrix[y - 2][x - 2] == 'A' && matrix[y - 3][x - 3] == 'S'
        {
            total += 1;
        }
    }

    if y > 2 {
        if matrix[y - 1][x] == 'M' && matrix[y - 2][x] == 'A' && matrix[y - 3][x] == 'S' {
            total += 1;
        }
    }

    if y > 2 && x < matrix[0].len() - 3 {
        if matrix[y - 1][x + 1] == 'M' && matrix[y - 2][x + 2] == 'A' && matrix[y - 3][x + 3] == 'S'
        {
            total += 1;
        }
    }

    if x > 2 {
        if matrix[y][x - 1] == 'M' && matrix[y][x - 2] == 'A' && matrix[y][x - 3] == 'S' {
            total += 1;
        }
    }

    if x < matrix[0].len() - 3 {
        if matrix[y][x + 1] == 'M' && matrix[y][x + 2] == 'A' && matrix[y][x + 3] == 'S' {
            total += 1;
        }
    }

    if y < matrix.len() - 3 && x > 2 {
        if matrix[y + 1][x - 1] == 'M' && matrix[y + 2][x - 2] == 'A' && matrix[y + 3][x - 3] == 'S'
        {
            total += 1;
        }
    }

    if y < matrix.len() - 3 {
        if matrix[y + 1][x] == 'M' && matrix[y + 2][x] == 'A' && matrix[y + 3][x] == 'S' {
            total += 1;
        }
    }

    if y < matrix.len() - 3 && x < matrix[0].len() - 3 {
        if matrix[y + 1][x + 1] == 'M' && matrix[y + 2][x + 2] == 'A' && matrix[y + 3][x + 3] == 'S'
        {
            total += 1;
        }
    }

    total
}

fn get_xmas_x_shape(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    if y < 1 || x < 1 || y > matrix.len() - 2 || x > matrix[0].len() - 2 {
        return 0;
    }

    if (matrix[y - 1][x - 1] == 'M'
        && matrix[y + 1][x + 1] == 'S'
        && matrix[y - 1][x + 1] == 'M'
        && matrix[y + 1][x - 1] == 'S')
        || (matrix[y - 1][x - 1] == 'M'
            && matrix[y + 1][x + 1] == 'S'
            && matrix[y - 1][x + 1] == 'S'
            && matrix[y + 1][x - 1] == 'M')
        || (matrix[y - 1][x - 1] == 'S'
            && matrix[y + 1][x + 1] == 'M'
            && matrix[y - 1][x + 1] == 'M'
            && matrix[y + 1][x - 1] == 'S')
        || (matrix[y - 1][x - 1] == 'S'
            && matrix[y + 1][x + 1] == 'M'
            && matrix[y - 1][x + 1] == 'S'
            && matrix[y + 1][x - 1] == 'M')
    {
        return 1;
    }

    0
}

fn part_one(matrix: &Vec<Vec<char>>) {
    let mut x_indexes = vec![];
    for y in 0..matrix.len() {
        let row = &matrix[y];
        for x in 0..row.len() {
            if matrix[y][x] == 'X' {
                x_indexes.push((y, x));
            }
        }
    }
    let mut total = 0;

    for (y, x) in x_indexes {
        total += get_xmas_arount(matrix, y, x);
    }
    println!("Part one: {}", total);
}

fn part_two(matrix: &Vec<Vec<char>>) {
    let mut a_indexes = vec![];
    for y in 0..matrix.len() {
        let row = &matrix[y];
        for x in 0..row.len() {
            if matrix[y][x] == 'A' {
                a_indexes.push((y, x));
            }
        }
    }
    let mut total = 0;

    for (y, x) in a_indexes {
        total += get_xmas_x_shape(matrix, y, x);
    }
    println!("Part two: {}", total);
}

fn main() {
    let input = get_input();
    let matrix = parse_input(&input);
    println!("Rust solution");
    part_one(&matrix);
    part_two(&matrix);
    println!("----------------------------------------");
}
