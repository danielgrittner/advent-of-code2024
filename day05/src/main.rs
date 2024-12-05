use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

macro_rules! hashset {
    ($( $elem:expr ),*) => {
        {
            let mut set = HashSet::new();
            $(set.insert($elem);)*
            set
        }
    };
}

fn read_input(path: &str) -> std::io::Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)> {
    let data = read_to_string(path)?;
    let lines = data
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut idx = 0;

    let mut conditions = HashMap::new();
    while idx < lines.len() && !lines[idx].is_empty() {
        let condition = lines[idx]
            .split("|")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (before, after) = (condition[0], condition[1]);
        conditions
            .entry(before)
            .and_modify(|afters: &mut HashSet<i32>| {
                afters.insert(after);
            })
            .or_insert_with(|| hashset![after]);
        idx += 1;
    }

    idx += 1;

    let mut runs = Vec::new();
    for j in idx..lines.len() {
        let run = lines[j]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        runs.push(run);
    }

    Ok((conditions, runs))
}

fn reorder_run_and_determine_if_was_valid(
    run: Vec<i32>,
    conditions: &HashMap<i32, HashSet<i32>>,
) -> (Vec<i32>, bool) {
    let mut reordered_run = run.clone();
    reordered_run.sort_by(|&a, &b| {
        if let Some(afters) = conditions.get(&a) {
            if afters.contains(&b) {
                return std::cmp::Ordering::Less;
            }
        }

        if let Some(afters) = conditions.get(&b) {
            if afters.contains(&a) {
                return std::cmp::Ordering::Greater;
            }
        }

        std::cmp::Ordering::Equal
    });
    let was_valid = reordered_run == run;
    (reordered_run, was_valid)
}

fn main() -> std::io::Result<()> {
    // let (conditions, runs) = read_input("example.txt")?;
    let (conditions, runs) = read_input("input.txt")?;

    let reordered_runs = runs
        .into_iter()
        .map(|run| reorder_run_and_determine_if_was_valid(run, &conditions))
        .collect::<Vec<(Vec<i32>, bool)>>();

    let task1 = reordered_runs
        .iter()
        .filter(|(_, was_valid)| *was_valid)
        .map(|(run, _)| run[run.len() / 2])
        .sum::<i32>();
    println!("Task 1: {task1}");

    let task2 = reordered_runs
        .iter()
        .filter(|(_, was_valid)| !*was_valid)
        .map(|(run, _)| run[run.len() / 2])
        .sum::<i32>();
    println!("Task 2: {task2}");

    Ok(())
}
