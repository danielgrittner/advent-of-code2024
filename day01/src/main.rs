use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input(path: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let data = read_to_string(path)?;
    let rows = data
        .split('\n')
        .map(|row| {
            row.split(' ')
                .filter(|&cell| !cell.is_empty())
                .map(|cell| cell.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Ok(rows.into_iter().map(|row| (row[0], row[1])).unzip())
}

fn main() -> std::io::Result<()> {
    // let (mut v1, mut v2) = read_input("example.txt")?;
    let (mut v1, mut v2) = read_input("input.txt")?;

    v1.sort_unstable();
    v2.sort_unstable();

    let task1 = v1
        .iter()
        .zip(v2.iter())
        .map(|(&x1, &x2)| (x1 - x2).abs())
        .sum::<i32>();

    println!("Task 1: {task1}");

    let lookup_table = v2.into_iter().fold(HashMap::new(), |mut ht, x| {
        ht.entry(x).and_modify(|cnt| *cnt += 1).or_insert(1);
        ht
    });

    let task2 = v1
        .iter()
        .map(|&x| {
            let cnt = lookup_table.get(&x).map(|&val| val).unwrap_or(0);
            cnt * x
        })
        .sum::<i32>();

    println!("Task 2: {task2}");

    Ok(())
}
