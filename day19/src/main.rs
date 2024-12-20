use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(c: char) -> Self {
        match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("Invalid color"),
        }
    }
}

fn read_data(path: &str) -> std::io::Result<(Vec<Vec<Color>>, Vec<Vec<Color>>)> {
    let data = read_to_string(path)?;
    let mut iter = data.split("\n");

    let patterns = iter.next().unwrap();
    let patterns = patterns
        .split(", ")
        .map(|pattern| pattern.chars().map(Color::from_char).collect())
        .collect();

    iter.next();

    let mut queries = Vec::new();
    while let Some(query) = iter.next() {
        queries.push(query.chars().map(Color::from_char).collect());
    }

    Ok((patterns, queries))
}

fn count_arrangements(
    pattern_lookup: &HashSet<Vec<Color>>,
    pattern_max_length: usize,
    query: &Vec<Color>,
) -> i64 {
    let mut dp = vec![0i64; query.len()];

    for idx in 0..query.len() {
        if idx > 0 && dp[idx - 1] == 0 {
            continue;
        }

        // found match for [0..idx-1], now try to find a
        // match starting at idx
        let mut pattern = Vec::with_capacity(pattern_max_length);
        for idx2 in idx..std::cmp::min(query.len(), idx + pattern_max_length) {
            pattern.push(query[idx2]);
            if pattern_lookup.contains(&pattern) {
                dp[idx2] += if idx > 0 { dp[idx - 1] } else { 1 };
            }
        }
    }

    dp[query.len() - 1]
}

fn main() -> std::io::Result<()> {
    // let (patterns, queries) = read_data("example.txt")?;
    let (patterns, queries) = read_data("input.txt")?;

    let pattern_max_length = patterns.iter().map(|pattern| pattern.len()).max().unwrap();
    let pattern_lookup = patterns.into_iter().collect::<HashSet<Vec<Color>>>();

    let start = std::time::Instant::now();
    let task1 = queries
        .iter()
        .filter(|query| count_arrangements(&pattern_lookup, pattern_max_length, *query) > 0)
        .count();
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = queries
        .iter()
        .map(|query| count_arrangements(&pattern_lookup, pattern_max_length, query))
        .sum::<i64>();
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
