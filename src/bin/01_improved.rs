use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();

        reader.lines().try_for_each(|line| {
            let line = line?;

            let split_vec: Vec<&str> = line.split_whitespace().collect();
            let left = split_vec[0].parse::<usize>()?;
            let right = split_vec[1].parse::<usize>()?;

            left_list.push(left);
            right_list.push(right);
            Ok(())
        })?;

        left_list.sort_unstable();
        right_list.sort_unstable();

        let sum = left_list
            .iter()
            .zip(right_list.iter())
            .map(|(&left, &right)| left.abs_diff(right))
            .sum();

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut left_list: HashMap<usize, usize> = HashMap::new();
        let mut right_list: HashMap<usize, usize> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let mut split_vec = line.split_whitespace();
            let left = split_vec
                .next()
                .ok_or_else(|| anyhow!("Missing right value"))?
                .parse::<usize>()?;
            let right = split_vec
                .next()
                .ok_or_else(|| anyhow!("Missing right value"))?
                .parse::<usize>()?;

            *left_list.entry(left).or_insert(0) += 1;
            *right_list.entry(right).or_insert(0) += 1;
        }

        let similarity_score = left_list.iter().fold(0, |acc, (&number, &count)| {
            acc + number * right_list.get(&number).unwrap_or(&0) * count
        });

        Ok(similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
