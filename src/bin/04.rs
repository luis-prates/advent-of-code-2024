use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn search_in_grid(input: &[Vec<char>], target: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];
    let mut total: usize = 0;

    let target_chars = target.chars().collect::<Vec<char>>();
    let rows = input.len();
    let cols = input[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                if is_match(input, &target_chars, row, col, dr, dc) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn is_match(
    input: &[Vec<char>],
    target_chars: &[char],
    start_row: usize,
    start_col: usize,
    dr: isize,
    dc: isize,
) -> bool {
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;

    for (i, &cr) in target_chars.iter().enumerate() {
        let r = start_row as isize + i as isize * dr;
        let c = start_col as isize + i as isize * dc;

        if r < 0 || r >= rows || c < 0 || c >= cols || input[r as usize][c as usize] != cr {
            return false;
        }
    }

    true
}

fn search_in_x_grid(input: &[Vec<char>], target: &str) -> usize {
    // let directions = [
    //     (1, 1),   // Down-Right
    //     (1, -1),  // Down-Left
    //     (-1, 1),  // Up-Right
    //     (-1, -1), // Up-Left
    // ];
    let mut total: usize = 0;

    let target_chars = target.chars().collect::<Vec<char>>();
    let rows = input.len();
    let cols = input[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if input[row][col] == 'A' && is_x_match(input, &target_chars, row, col) {
                total += 1;
            }
        }
    }

    total
}

fn is_x_match(
    input: &[Vec<char>],
    target_chars: &[char],
    start_row: usize,
    start_col: usize,
) -> bool {
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;
    let mut sorted_target: Vec<char> = target_chars.to_vec();
    sorted_target.sort();

    let up_left_row = start_row as isize - 1;
    let up_left_col = start_col as isize - 1;

    let down_right_row = start_row as isize + 1;
    let down_right_col = start_col as isize + 1;

    let up_right_row = start_row as isize - 1;
    let up_right_col = start_col as isize + 1;

    let down_left_row = start_row as isize + 1;
    let down_left_col = start_col as isize - 1;

    if up_left_row < 0 || down_right_row >= rows || up_left_col < 0 || down_right_col >= cols {
        return false;
    } else if ((input[up_left_row as usize][up_left_col as usize] == 'M'
        && input[down_right_row as usize][down_right_col as usize] == 'S')
        || input[up_left_row as usize][up_left_col as usize] == 'S'
            && input[down_right_row as usize][down_right_col as usize] == 'M')
        && ((input[up_right_row as usize][up_right_col as usize] == 'M'
            && input[down_left_row as usize][down_left_col as usize] == 'S')
            || input[up_right_row as usize][up_right_col as usize] == 'S'
                && input[down_left_row as usize][down_left_col as usize] == 'M')
    {
        return true;
    }

    false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        // let buffer = reader.fill_buf()?;
        // let line_end = buffer
        //     .iter()
        //     .position(|&b| b == b'\n')
        //     .unwrap_or(buffer.len());

        let full_string = reader
            .lines()
            .map(|line| line.map(|s| s.chars().collect::<Vec<char>>()))
            .collect::<Result<Vec<Vec<char>>, _>>()?;

        let answer = search_in_grid(&full_string, "XMAS");

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(&mut BufReader::new(TEST.as_bytes()))?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(&mut input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let full_string = reader
            .lines()
            .map(|line| line.map(|s| s.chars().collect::<Vec<char>>()))
            .collect::<Result<Vec<Vec<char>>, _>>()?;

        let answer = search_in_x_grid(&full_string, "XMAS");

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
