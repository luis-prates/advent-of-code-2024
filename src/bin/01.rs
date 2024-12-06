use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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
        let mut sum = 0;
        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();

        // let answer = reader.lines().flatten().count();
        let _ = reader.lines().try_for_each(|line| -> Result<()> {
            let binding = line?;
            let split_vec = binding.split_whitespace().collect::<Vec<&str>>();
            left_list.push(split_vec[0].parse::<usize>()?);
            right_list.push(split_vec[1].parse::<usize>()?);
            Ok(())
        });

        left_list.sort();
        right_list.sort();
        left_list.reverse();
        right_list.reverse();

        while !left_list.is_empty() {
            let left_min = left_list
                .iter()
                .min()
                .ok_or_else(|| anyhow!("left_list is empty"))?;
            let right_min = right_list
                .iter()
                .min()
                .ok_or_else(|| anyhow!("right_list is empty"))?;
            sum += left_min.abs_diff(*right_min);
            left_list.pop();
            right_list.pop();
        }

        // for line in &my_answer {
        //     let split_vec = line.split_whitespace().collect::<Vec<&str>>();
        //     sum += split_vec[0]
        //         .parse::<usize>()
        //         .unwrap()
        //         .abs_diff(split_vec[1].parse::<usize>().unwrap());
        // }
        // println!("{:?}", my_answer);
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
        let mut similarity_score = 0;
        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();

        // let answer = reader.lines().flatten().count();
        let _ = reader.lines().try_for_each(|line| -> Result<()> {
            let binding = line?;
            let split_vec = binding.split_whitespace().collect::<Vec<&str>>();
            left_list.push(split_vec[0].parse::<usize>()?);
            right_list.push(split_vec[1].parse::<usize>()?);
            Ok(())
        });

        for number in left_list {
            let number_count = right_list.iter().filter(|&&x| x == number).count();
            similarity_score += number * number_count;
        }

        Ok(similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
