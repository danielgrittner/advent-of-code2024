use rayon::prelude::*;
use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<(Vec<[i32; 5]>, Vec<[i32; 5]>)> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let txt = read_to_string(path)?;
    let mut iter = txt.split("\n");

    while let Some(row) = iter.next() {
        let mut rows = vec![row.to_string()];
        while let Some(row) = iter.next() {
            if row.is_empty() {
                break;
            }
            rows.push(row.to_string());
        }

        let mut counts = [-1; 5];
        for idx in 0..rows.len() {
            for (idx2, c) in rows[idx].chars().enumerate() {
                if c == '#' {
                    counts[idx2] += 1;
                }
            }
        }

        let is_key = rows[0].starts_with(".");
        if is_key {
            keys.push(counts);
        } else {
            locks.push(counts);
        }
    }

    Ok((keys, locks))
}

fn main() -> std::io::Result<()> {
    // let (keys, locks) = read_data("example.txt")?;
    let (keys, locks) = read_data("input.txt")?;

    let start_time = std::time::Instant::now();
    let task1 = keys
        .par_iter()
        .map(|key| {
            let mut count = 0;
            for lock in &locks {
                let is_match = key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5);
                if is_match {
                    count += 1;
                }
            }
            count
        })
        .sum::<i32>();
    let duration = start_time.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    Ok(())
}
