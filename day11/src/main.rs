use std::collections::HashMap;
use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(path)?
        .split(" ")
        .map(|num| num.parse::<i64>().unwrap())
        .collect())
}

fn to_digits(mut num: i64) -> Vec<i64> {
    if num == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

fn to_num(digits: &[i64]) -> i64 {
    let mut offset = 1;
    let mut num = 0;
    for &digit in digits {
        num += offset * digit;
        offset *= 10;
    }
    num
}

fn simulate(iniital: &Vec<i64>, rounds: usize) -> usize {
    let mut current = iniital
        .iter()
        .map(|&num| (num, 1))
        .collect::<HashMap<i64, usize>>();
    for _ in 0..rounds {
        let mut next = HashMap::new();

        for (stone, stone_cnt) in current.into_iter() {
            if stone == 0 {
                next.entry(1)
                    .and_modify(|cnt| *cnt += stone_cnt)
                    .or_insert(stone_cnt);
                continue;
            }

            let digits = to_digits(stone);
            let n = digits.len();
            if n % 2 == 0 {
                next.entry(to_num(&digits[..n / 2]))
                    .and_modify(|cnt| *cnt += stone_cnt)
                    .or_insert(stone_cnt);
                next.entry(to_num(&digits[n / 2..]))
                    .and_modify(|cnt| *cnt += stone_cnt)
                    .or_insert(stone_cnt);
                continue;
            }

            next.entry(stone * 2024)
                .and_modify(|cnt| *cnt += stone_cnt)
                .or_insert(stone_cnt);
        }

        current = next;
    }
    current.into_iter().map(|(_, cnt)| cnt).sum::<usize>()
}

fn main() -> std::io::Result<()> {
    // let data = read_data("example.txt")?;
    let data = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = simulate(&data, 25);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = simulate(&data, 75);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
