use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut total: usize = 0;
        reader.lines().try_for_each(|line| {
            let line = line?;

            let re = Regex::new(r"mul\\((\\d+),(\\d+)\\)")?;
            for cap in re.captures_iter(&line) {
                total += cap[1].parse::<usize>()? * cap[2].parse::<usize>()?;
            }

            Ok(())
        })?;
        Ok(total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total: usize = 0;
        let full_text = reader.lines().map_while(Result::ok).collect::<String>();

        let re_between = Regex::new(r"don't\(\).*?do\(\)")?;
        let result = re_between.replace_all(&full_text, "");
        let re_to_end = Regex::new(r"don't\(\).*$")?;
        let final_result = re_to_end.replace_all(&result, "");

        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
        for cap in re.captures_iter(&final_result) {
            total += cap[1].parse::<usize>()? * cap[2].parse::<usize>()?;
        }

        Ok(total)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
