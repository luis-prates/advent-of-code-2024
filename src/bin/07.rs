use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODO: Add the test input

fn recursive_check(result: usize, operands: &Vec<usize>, current: usize, cumul: usize) -> bool {
    if current == operands.len() {
        return cumul == result;
    }

    if recursive_check(result, operands, current + 1, cumul + operands[current]) {
        true
    } else {
        recursive_check(result, operands, current + 1, cumul * operands[current])
    }
}

fn recursive_check_part2(
    result: usize,
    operands: &Vec<usize>,
    current: usize,
    cumul: usize,
) -> bool {
    if current == operands.len() {
        return cumul == result;
    }

    if recursive_check_part2(result, operands, current + 1, cumul + operands[current])
        || recursive_check_part2(result, operands, current + 1, cumul * operands[current])
    {
        true
    } else {
        let mut cumul_string = cumul.to_string();
        let current_string = operands[current].to_string();
        cumul_string.push_str(&current_string);
        recursive_check_part2(result, operands, current + 1, cumul_string.parse().unwrap())
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        let mut results: Vec<usize> = vec![];
        let mut operands: Vec<Vec<usize>> = vec![];
        // TODO: Solve Part 1 of the puzzle
        reader.lines().try_for_each(|line| {
            let line = line?;
            let split_string = line.split(':').collect::<Vec<_>>();
            results.push(split_string[0].parse().unwrap());
            operands.push(
                split_string[1]
                    .split_whitespace()
                    .map(|operand| operand.parse().unwrap())
                    .collect(),
            );
            Ok(())
        })?;

        for (index, result) in results.iter().enumerate() {
            let valid = recursive_check(*result, &operands[index], 1, operands[index][0]);
            if valid {
                total += result;
            }
        }

        Ok(total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        let mut results: Vec<usize> = vec![];
        let mut operands: Vec<Vec<usize>> = vec![];
        // TODO: Solve Part 1 of the puzzle
        reader.lines().try_for_each(|line| {
            let line = line?;
            let split_string = line.split(':').collect::<Vec<_>>();
            results.push(split_string[0].parse().unwrap());
            operands.push(
                split_string[1]
                    .split_whitespace()
                    .map(|operand| operand.parse().unwrap())
                    .collect(),
            );
            Ok(())
        })?;

        for (index, result) in results.iter().enumerate() {
            let valid = recursive_check_part2(*result, &operands[index], 1, operands[index][0]);
            if valid {
                total += result;
            }
        }

        Ok(total)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
