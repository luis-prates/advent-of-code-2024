use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

struct Tile {
    character: char,
    is_visited: bool,
}

struct Guard {
    position: (i32, i32),
    out: bool,
    direction: (i32, i32),
}

fn get_guard_position(grid: &[Vec<Tile>]) -> Option<(i32, i32)> {
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|col| col.character == '^') {
            return Some((row_idx as i32, col_index as i32));
        }
    }
    None
}

fn get_distinct_positions(grid: &mut [Vec<Tile>]) -> usize {
    let mut total_visited = 0;
    let row_len = grid.len() as i32;
    let col_len = grid[0].len() as i32;

    let mut guard: Guard = Guard {
        position: match get_guard_position(grid) {
            Some(pos) => pos,
            None => return 0,
        },
        out: false,
        direction: (-1, 0),
    };

    while !guard.out {
        let current_position = &mut grid[guard.position.0 as usize][guard.position.1 as usize];

        if !current_position.is_visited {
            current_position.is_visited = true;
            total_visited += 1;
        }

        let next_guard_move = (
            (guard.position.0 + guard.direction.0),
            guard.position.1 + guard.direction.1,
        );
        if next_guard_move.0.is_negative()
            || next_guard_move.0 >= row_len
            || next_guard_move.1.is_negative()
            || next_guard_move.1 >= col_len
        {
            guard.out = true;
            break;
        }

        let next_position = &grid[next_guard_move.0 as usize][next_guard_move.1 as usize];
        if next_position.character == '#' {
            guard.direction = (guard.direction.1, -guard.direction.0);
        }
        guard.position.0 += guard.direction.0;
        guard.position.1 += guard.direction.1;
        // if next_position.character.eq(&'#') {
        //     guard.direction = (-guard.direction.1, guard.direction.0);
        // }
    }

    total_visited
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut grid = reader
            .lines()
            .map(|line| {
                line.map(|s| {
                    s.chars()
                        .map(|c| Tile {
                            character: c,
                            is_visited: false,
                        })
                        .collect::<Vec<Tile>>()
                })
            })
            .collect::<Result<Vec<Vec<Tile>>, _>>()?;

        let answer = get_distinct_positions(&mut grid);

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
