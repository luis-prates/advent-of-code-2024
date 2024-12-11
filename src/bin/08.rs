use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut char_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        // TODO: Solve Part 1 of the puzzle
        let strings = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
        for (row, s) in strings.iter().enumerate() {
            for (col, ch) in s.chars().enumerate() {
                if ch != '.' {
                    char_positions.entry(ch).or_default().push((row, col));
                }
            }
        }

        // println!("chars: {:?}", char_positions.keys());

        let mut unique_positions: HashSet<(isize, isize)> = HashSet::new();

        for (_, positions) in char_positions {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    let x_distance = x2 as isize - x1 as isize;
                    let y_distance = y2 as isize - y1 as isize;

                    // println!(
                    //     "For char {} between position {:?} and {:?} x distance is {} and y is {}",
                    //     entry_char, positions[i], positions[j], x_distance, y_distance
                    // );

                    if positions[i].0 as isize - x_distance < strings.len() as isize
                        && positions[i].0 as isize - x_distance >= 0
                        && positions[i].1 as isize - y_distance < strings[0].len() as isize
                        && positions[i].1 as isize - y_distance >= 0
                    {
                        // println!(
                        //     "Antinode would be at ({}, {})",
                        //     positions[i].0 as isize - x_distance,
                        //     positions[i].1 as isize - y_distance
                        // );

                        unique_positions.insert((
                            positions[i].0 as isize - x_distance,
                            positions[i].1 as isize - y_distance,
                        ));
                    }
                    if positions[j].0 as isize + x_distance < strings.len() as isize
                        && positions[j].0 as isize + x_distance >= 0
                        && positions[j].1 as isize + y_distance < strings[0].len() as isize
                        && positions[j].1 as isize + y_distance >= 0
                    {
                        // println!(
                        //     "Antinode would be at ({}, {})",
                        //     positions[j].0 as isize + x_distance,
                        //     positions[j].1 as isize + y_distance
                        // );

                        unique_positions.insert((
                            positions[j].0 as isize + x_distance,
                            positions[j].1 as isize + y_distance,
                        ));
                    }
                }
            }
        }

        let total = unique_positions.len();

        Ok(total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut char_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        // TODO: Solve Part 1 of the puzzle
        let strings = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
        for (row, s) in strings.iter().enumerate() {
            for (col, ch) in s.chars().enumerate() {
                if ch != '.' {
                    char_positions.entry(ch).or_default().push((row, col));
                }
            }
        }

        // println!("chars: {:?}", char_positions.keys());

        let mut unique_positions: HashSet<(isize, isize)> = HashSet::new();

        for (_, positions) in char_positions {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    let x_distance = x2 as isize - x1 as isize;
                    let y_distance = y2 as isize - y1 as isize;

                    // println!(
                    //     "For char {} between position {:?} and {:?} x distance is {} and y is {}",
                    //     entry_char, positions[i], positions[j], x_distance, y_distance
                    // );
                    unique_positions.insert((positions[i].0 as isize, positions[i].1 as isize));
                    unique_positions.insert((positions[j].0 as isize, positions[j].1 as isize));

                    let mut multiplier = 1;
                    let mut should_stop_one = false;
                    let mut should_stop_two = false;
                    while !should_stop_one || !should_stop_two {
                        if positions[i].0 as isize - x_distance * multiplier
                            < strings.len() as isize
                            && positions[i].0 as isize - x_distance * multiplier >= 0
                            && positions[i].1 as isize - y_distance * multiplier
                                < strings[0].len() as isize
                            && positions[i].1 as isize - y_distance * multiplier >= 0
                        {
                            // println!(
                            //     "Antinode would be at ({}, {})",
                            //     positions[i].0 as isize - x_distance,
                            //     positions[i].1 as isize - y_distance
                            // );

                            unique_positions.insert((
                                positions[i].0 as isize - x_distance * multiplier,
                                positions[i].1 as isize - y_distance * multiplier,
                            ));
                        } else {
                            should_stop_one = true;
                        }
                        if positions[j].0 as isize + x_distance * multiplier
                            < strings.len() as isize
                            && positions[j].0 as isize + x_distance * multiplier >= 0
                            && positions[j].1 as isize + y_distance * multiplier
                                < strings[0].len() as isize
                            && positions[j].1 as isize + y_distance * multiplier >= 0
                        {
                            // println!(
                            //     "Antinode would be at ({}, {})",
                            //     positions[j].0 as isize + x_distance,
                            //     positions[j].1 as isize + y_distance
                            // );

                            unique_positions.insert((
                                positions[j].0 as isize + x_distance * multiplier,
                                positions[j].1 as isize + y_distance * multiplier,
                            ));
                        } else {
                            should_stop_two = true;
                        }
                        multiplier += 1;
                    }
                }
            }
        }

        let total = unique_positions.len();

        Ok(total)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
