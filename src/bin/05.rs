use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
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
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut total = 0;
        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf);
        let parts: Vec<&str> = buf.split("\n\n").collect();

        let first_part: Vec<Vec<i32>> = parts[0]
            .lines()
            .map(|line| {
                line.split('|')
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let second_part: Vec<Vec<i32>> = parts[1]
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        for update in second_part {
            let mut valid = true;

            for order in first_part.iter() {
                let updated_first = update.iter().position(|&x| x == order[0]);
                let updated_second = update.iter().position(|&x| x == order[1]);

                if let (Some(first_value), Some(second_value)) = (updated_first, updated_second) {
                    if first_value > second_value {
                        valid = false;
                        break;
                    }
                } else {
                    continue;
                }
            }
            if valid {
                if let Some(value) = update.get(update.len() / 2) {
                    total += *value as usize;
                }
            }
        }

        Ok(total)
    }

    // test input
    assert_eq!(143, part1(&mut BufReader::new(TEST.as_bytes()))?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(&mut input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: &mut R) -> Result<usize> {
        let mut total = 0;
        let mut buf = String::new();

        let _ = reader.read_to_string(&mut buf);
        let parts: Vec<&str> = buf.split("\n\n").collect();

        let first_part: Vec<Vec<i32>> = parts[0]
            .lines()
            .map(|line| {
                line.split('|')
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let second_part: Vec<Vec<i32>> = parts[1]
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        for update in second_part {
            let mut valid = false;
            let mut corrected = false;
            let mut new_update = update.clone();

            while !valid {
                valid = true;

                for order in first_part.iter() {
                    let updated_first = new_update.iter().position(|&x| x == order[0]);
                    let updated_second = new_update.iter().position(|&x| x == order[1]);

                    if let (Some(first_value), Some(second_value)) = (updated_first, updated_second)
                    {
                        if first_value > second_value {
                            valid = false;
                            corrected = true;
                            new_update.swap(first_value, second_value);
                        }
                    } else {
                        continue;
                    }
                }
            }
            if valid && corrected {
                if let Some(value) = new_update.get(new_update.len() / 2) {
                    total += *value as usize;
                }
            }
        }

        Ok(total)
    }

    // test input
    assert_eq!(123, part2(&mut BufReader::new(TEST.as_bytes()))?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(&mut input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
