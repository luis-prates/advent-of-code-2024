use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

enum Direction {
    Increasing,
    Decreasing,
}

type Report = Vec<i32>;

fn check_safety(report: &Report) -> Result<()> {
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(anyhow!("{} {} switched to increasing", a, b));
                }
                Some(Direction::Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(anyhow!("{} {} diff value is {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(anyhow!("{} {} diff value is {}", a, b, diff.abs()));
                    } else {
                        direction = Some(Direction::Decreasing);
                        continue;
                    }
                }
            },
            1 => match direction {
                Some(Direction::Decreasing) => {
                    return Err(anyhow!("{} {} switched to decreasing", a, b));
                }
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(anyhow!("{} {} diff value is {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(anyhow!("{} {} diff value is {}", a, b, diff.abs()));
                    } else {
                        direction = Some(Direction::Increasing);
                        continue;
                    }
                }
            },
            0 => {
                return Err(anyhow!("{}, {} diff was 0", a, b));
            }
            _ => {
                panic!("Should never have a non -1,1,0 number");
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut safe_counter: usize = 0;

        reader.lines().try_for_each(|line| {
            let line = line?;

            let split_vec: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?;

            if check_safety(&split_vec).is_ok() {
                safe_counter += 1;
            }
            Ok(())
        })?;
        Ok(safe_counter)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_counter: usize = 0;

        reader.lines().try_for_each(|line| {
            let line = line?;

            let split_vec: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?;

            if check_safety(&split_vec).is_err() {
                for index in 0..split_vec.len() {
                    let mut new_report = split_vec.clone();
                    new_report.remove(index);
                    if check_safety(&new_report).is_ok() {
                        safe_counter += 1;
                        break;
                    } else {
                        continue;
                    }
                }
            } else {
                safe_counter += 1;
            }
            Ok(())
        })?;
        Ok(safe_counter)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
