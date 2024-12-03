use regex::Regex;
use std::fs::read_to_string;

fn read_input(path: &str) -> std::io::Result<String> {
    let data = read_to_string(path)?;
    Ok(data.to_string())
}

fn main() -> std::io::Result<()> {
    // let data = read_input("example.txt")?;
    // let data = read_input("example2.txt")?;
    let data = read_input("input.txt")?;

    let mul_re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let digits_re = Regex::new(r"(?<x>\d+),(?<y>\d+)").unwrap();

    let task1 = mul_re
        .find_iter(&data)
        .map(|mul| mul.as_str())
        .map(|mul| {
            let mut it = digits_re.captures_iter(mul);
            let caps = it.next().unwrap();

            let x = caps["x"].parse::<i32>().unwrap();
            let y = caps["y"].parse::<i32>().unwrap();

            x * y
        })
        .sum::<i32>();
    println!("Task 1: {task1}");

    let mul_do_dont_re = Regex::new(r"(mul\(\d+,\d+\)|don't\(\)|do\(\))").unwrap();

    let mut is_enabled = true;
    let mut task2 = 0;
    for m in mul_do_dont_re.find_iter(&data) {
        let m_str = m.as_str();

        if m_str == "do()" {
            is_enabled = true;
            continue;
        }
        if m_str == "don't()" {
            is_enabled = false;
            continue;
        }

        if !is_enabled {
            continue;
        }

        let mut it = digits_re.captures_iter(m_str);
        let caps = it.next().unwrap();

        let x = caps["x"].parse::<i32>().unwrap();
        let y = caps["y"].parse::<i32>().unwrap();

        task2 += x * y;
    }
    println!("Task 2: {task2}");

    Ok(())
}
