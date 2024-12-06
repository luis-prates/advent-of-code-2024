use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
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

// Function to build the directed graph
fn build_graph(pairs: &[(i32, i32)]) -> HashMap<i32, Vec<i32>> {
    let mut graph = HashMap::new();

    for &(a, b) in pairs {
        graph.entry(a).or_insert_with(Vec::new).push(b);
        graph.entry(b).or_insert_with(Vec::new); // Ensure every node exists in the graph
    }

    graph
}

// Function to check if an update is valid
fn is_valid_order(graph: &HashMap<i32, Vec<i32>>, update: &[i32]) -> bool {
    // Extract the subgraph relevant to the update
    let nodes: HashSet<i32> = update.iter().copied().collect();
    let mut indegree = HashMap::new();
    let mut subgraph = HashMap::new();

    for &node in &nodes {
        indegree.insert(node, 0);
        subgraph.insert(node, vec![]);
    }

    for (&from, edges) in graph {
        if nodes.contains(&from) {
            for &to in edges {
                if nodes.contains(&to) {
                    if let Some(neighbors) = subgraph.get_mut(&from) {
                        neighbors.push(to);
                    }
                    *indegree.entry(to).or_insert(0) += 1;
                }
            }
        }
    }

    // Topological sort to validate the order
    let mut queue: VecDeque<i32> = indegree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = vec![];

    while let Some(node) = queue.pop_front() {
        sorted_order.push(node);

        if let Some(neighbors) = subgraph.get(&node) {
            for &neighbor in neighbors {
                if let Some(count) = indegree.get_mut(&neighbor) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check if the update matches the sorted order
    let sorted_set: HashSet<i32> = sorted_order.iter().copied().collect();
    let update_set: HashSet<i32> = update.iter().copied().collect();

    if sorted_set != update_set {
        return false;
    }

    let mut index_map = HashMap::new();
    for (i, &val) in update.iter().enumerate() {
        index_map.insert(val, i);
    }

    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let a = update[i];
            let b = update[j];
            if let Some(edges) = subgraph.get(&a) {
                if !edges.contains(&b) {
                    return false;
                }
            }
        }
    }

    true
}

fn fix_invalid_order(graph: &HashMap<i32, Vec<i32>>, update: &[i32]) -> Vec<i32> {
    // Extract the subgraph relevant to the update
    let nodes: HashSet<i32> = update.iter().copied().collect();
    let mut indegree = HashMap::new();
    let mut subgraph = HashMap::new();

    for &node in &nodes {
        indegree.insert(node, 0);
        subgraph.insert(node, vec![]);
    }

    for (&from, edges) in graph {
        if nodes.contains(&from) {
            for &to in edges {
                if nodes.contains(&to) {
                    if let Some(neighbors) = subgraph.get_mut(&from) {
                        neighbors.push(to);
                    }
                    *indegree.entry(to).or_insert(0) += 1;
                }
            }
        }
    }

    // Topological sort to validate the order
    let mut queue: VecDeque<i32> = indegree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = vec![];

    while let Some(node) = queue.pop_front() {
        sorted_order.push(node);

        if let Some(neighbors) = subgraph.get(&node) {
            for &neighbor in neighbors {
                if let Some(count) = indegree.get_mut(&neighbor) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted_order
}

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

        let first_part: Vec<(i32, i32)> = parts[0]
            .lines()
            .map(|line| {
                let mut nums = line.split('|').map(|s| s.parse::<i32>());
                match (nums.next(), nums.next()) {
                    (Some(Result::Ok(a)), Some(Result::Ok(b))) => Result::Ok((a, b)),
                    _ => Err(anyhow!("Line does not contain exactly two valid integers")),
                }
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

        let graph = build_graph(&first_part);

        // Check each update
        for (i, update) in second_part.iter().enumerate() {
            let is_valid = is_valid_order(&graph, update);
            if is_valid {
                // println!("Update {} is in the correct order.", i + 1);
                if let Some(value) = update.get(update.len() / 2) {
                    total += *value as usize;
                }
            } else {
                println!("Update {} is NOT in the correct order.", i + 1);
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
        let mut reordered_pages: Vec<Vec<i32>> = vec![];

        let _ = reader.read_to_string(&mut buf);
        let parts: Vec<&str> = buf.split("\n\n").collect();

        let first_part: Vec<(i32, i32)> = parts[0]
            .lines()
            .map(|line| {
                let mut nums = line.split('|').map(|s| s.parse::<i32>());
                match (nums.next(), nums.next()) {
                    (Some(Result::Ok(a)), Some(Result::Ok(b))) => Result::Ok((a, b)),
                    _ => Err(anyhow!("Line does not contain exactly two valid integers")),
                }
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
                let updated_first = update.iter().position(|&x| x == order.0);
                let updated_second = update.iter().position(|&x| x == order.1);

                if let (Some(first_value), Some(second_value)) = (updated_first, updated_second) {
                    if first_value > second_value {
                        valid = false;

                        break;
                    }
                } else {
                    continue;
                }
            }
            if !valid {
                reordered_pages.push(update.clone());
            }
        }

        let graph = build_graph(&first_part);

        for (i, update) in reordered_pages.iter().enumerate() {
            let is_valid = fix_invalid_order(&graph, update);
            if let Some(value) = is_valid.get(is_valid.len() / 2) {
                println!("Update {} corrected is {:?}", i + 1, is_valid);
                total += *value as usize;
            } else {
                println!("Update {} is NOT in the correct order.", i + 1);
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
