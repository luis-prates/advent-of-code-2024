use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = match reader.lines().map_while(Result::ok).next() {
            Some(line) => line,
            None => return Err(anyhow!("No lines found in the input")),
        };
        let mut id = 0;
        let mut full_vec: Vec<isize> = vec![];

        let _: Vec<_> = answer
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                let mut value = match c.to_digit(10) {
                    Some(v) => v,
                    None => return Err(anyhow!("Invalid digit found in the input")),
                };
                if idx % 2 != 0 {
                    while value > 0 {
                        full_vec.push(-1);
                        value -= 1;
                    }
                    Ok(())
                } else {
                    while value > 0 {
                        full_vec.push(id);
                        value -= 1;
                    }
                    id += 1;
                    Ok(())
                }
            })
            .collect();

        let mut copy_vec = full_vec.clone();

        for (idx, _) in full_vec.iter().enumerate().rev() {
            let (empty_index, _) = match copy_vec.iter().enumerate().find(|(_, el)| **el == -1) {
                Some(val) => val,
                None => return Err(anyhow!("No empty index found")),
            };

            // println!(
            //     "Swapping {} at index {} for {} at index {}",
            //     cha, empty_index, element, idx
            // );
            copy_vec.swap(empty_index, idx);
        }

        copy_vec.remove(0);

        let total = copy_vec
            .iter()
            .filter(|el| **el > -1)
            .enumerate()
            .map(|(idx, element)| idx * *element as usize)
            .sum();

        Ok(total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
