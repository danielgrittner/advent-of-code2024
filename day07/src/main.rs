use std::fs::read_to_string;
use rayon::prelude::*;

fn read_input(path: &str) -> std::io::Result<Vec<(i64, Vec<i64>)>> {
    let data = read_to_string(path)?;

    Ok(data
        .split("\n")
        .map(|equation| {
            let mut nums = equation
                .split(" ")
                .map(|num| {
                    if num.ends_with(":") {
                        num[..num.len() - 1].parse::<i64>().unwrap()
                    } else {
                        num.parse::<i64>().unwrap()
                    }
                })
                .collect::<Vec<i64>>();

            let target = nums.remove(0);
            (target, nums)
        })
        .collect())
}

fn evaluate(idx: usize, target: i64, current: i64, shift: i64, nums: &Vec<i64>, is_part2: bool) -> bool {
    if current > target {
        return false;
    }

    if idx >= nums.len() {
        return current == target;
    }

    evaluate(idx + 1, target, current + nums[idx], shift * 10, nums, is_part2)
        || evaluate(idx + 1, target, current * nums[idx], shift * 10, nums, is_part2)
        || (is_part2 && evaluate(idx + 1, target, current * shift + nums[idx], shift * 10, nums, is_part2))
}

fn main() -> std::io::Result<()> {
    // let data = read_input("example.txt")?;
    let data = read_input("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = data
        .par_iter()
        .filter(|(target, nums)| evaluate(1, *target, nums[0], 1, nums, false))
        .map(|(target, _)| target)
        .sum::<i64>();
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = data
        .par_iter()
        .filter(|(target, nums)| evaluate(1, *target, nums[0], 1, nums, true))
        .map(|(target, _)| target)
        .sum::<i64>();
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
