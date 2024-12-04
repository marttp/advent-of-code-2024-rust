mod common;

use common::split_lines;

fn main() {
    let input = include_str!("./input4.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let table = input
        .into_iter()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let m = table.len();
    let n = table[0].len();
    let mut count = 0;
    /*
    it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X.
    One way to achieve that is like this:
        M.S
        .A.
        M.S
    but it can be below pattern as well
        S.S
        .A.
        M.M
    */
    for r in 0..m - 2 {
        for c in 0..n - 2 {
            // Scan each character
            // Let say first character on first line of block 3 x 3
            if check_xmas_pattern(&table, r, c) {
                count += 1;
            }
        }
    }

    count
}

fn check_xmas_pattern(table: &[Vec<char>], r: usize, c: usize) -> bool {
    // Overlapped A is must
    // Don't forget, X-MAS pattern check from first line of block 3 x 3
    if table[r + 1][c + 1] != 'A' {
        return false;
    }

    // Cross from top-left => bottom-right
    let first_diagonal_rule = (table[r][c] == 'M' && table[r + 2][c + 2] == 'S') ||
        (table[r][c] == 'S' && table[r + 2][c + 2] == 'M');
    // Cross from top-right => bottom-left
    let second_diagonal_rule = (table[r + 2][c] == 'M' && table[r][c + 2] == 'S') ||
        (table[r + 2][c] == 'S' && table[r][c + 2] == 'M');

    return first_diagonal_rule && second_diagonal_rule;
}