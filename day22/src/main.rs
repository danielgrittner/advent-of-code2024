use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|num| num.parse::<i64>().unwrap())
        .collect())
}

const PRUNE: i64 = 16777216;

fn generate_pseudorandom(seed: i64, rounds: i64) -> Vec<i64> {
    let mut sequence = vec![seed];

    let mut num = seed;
    for _ in 0..rounds {
        num ^= num * 64;
        num %= PRUNE;

        num ^= num / 32;
        num %= PRUNE;

        num ^= num * 2048;
        num %= PRUNE;

        sequence.push(num);
    }

    sequence
}

fn get_last_digits(nums: Vec<i64>) -> Vec<i64> {
    nums.iter().map(|num| num % 10).collect()
}

fn group_bananas_by_subsequent_diffs(secret_nums: Vec<i64>) -> HashMap<[i64; 4], i64> {
    let digits = get_last_digits(secret_nums);

    let mut seq = [
        digits[0],
        digits[1] - digits[0],
        digits[2] - digits[1],
        digits[3] - digits[2],
    ];

    let mut groups = HashMap::new();
    groups.insert(seq, digits[3]);

    for idx in 4..digits.len() {
        seq.rotate_left(1);
        seq[3] = digits[idx] - digits[idx - 1];

        // only the first appearance of the sequence matters
        let weight = digits[idx];
        groups.entry(seq).or_insert(weight);
    }

    groups
}

fn main() -> std::io::Result<()> {
    // let data = read_data("example.txt")?;
    // let data = read_data("example2.txt")?;
    let data = read_data("input.txt")?;

    let start_time = std::time::Instant::now();
    let task1 = data
        .par_iter()
        .map(|&num| generate_pseudorandom(num, 2000)[2000])
        .sum::<i64>();
    let duration = start_time.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start_time = std::time::Instant::now();
    let groups = data
        .par_iter()
        .map(|&num| generate_pseudorandom(num, 2000))
        .map(group_bananas_by_subsequent_diffs)
        .reduce(
            || HashMap::new(),
            |mut acc, mut other| {
                if other.len() > acc.len() {
                    std::mem::swap(&mut acc, &mut other);
                }
                for (seq, price) in other {
                    acc.entry(seq)
                        .and_modify(|price_acc| *price_acc += price)
                        .or_insert(price);
                }
                acc
            },
        );
    let task2 = groups
        .into_iter()
        .map(|(_, total_price_for_sequence)| total_price_for_sequence)
        .max()
        .unwrap();
    let duration = start_time.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
