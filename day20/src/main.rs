use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<Vec<char>>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| row.chars().collect())
        .collect())
}

fn calculate_path(
    field: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    let mut path = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        path.push((x, y));

        if (x, y) == end {
            break;
        }

        for (ox, oy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x2 = x as i32 + ox;
            let y2 = y as i32 + oy;

            if 0 <= x2 && x2 < (field.len() as i32) && 0 <= y2 && y2 < (field[0].len() as i32) {
                let x2 = x2 as usize;
                let y2 = y2 as usize;

                if !visited[x2][y2] && field[x2][y2] != '#' {
                    queue.push_back((x2, y2));
                    visited[x2][y2] = true;
                }
            }
        }
    }

    path
}

fn l1_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    ((p1.0 as i64 - p2.0 as i64).abs() + (p1.1 as i64 - p2.1 as i64).abs()) as usize
}

fn count_cheats(
    field: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    cheat_threshold: usize,
    l1_distance_threshold: usize,
) -> usize {
    let path = calculate_path(field, start, end);

    let threshold = path.len() - 1 - cheat_threshold;

    (0..path.len())
        .into_par_iter()
        .map(|idx| {
            let mut count = 0;
            let remaining_dist = path.len() - idx - 1;

            for idx2 in 0..idx {
                let cheat_dist = l1_distance(path[idx2], path[idx]);
                if cheat_dist <= l1_distance_threshold {
                    let steps = remaining_dist + cheat_dist + idx2;
                    if steps <= threshold {
                        count += 1;
                    }
                }
            }

            count
        })
        .sum::<usize>()
}

fn main() -> std::io::Result<()> {
    // let data = read_data("example.txt")?;
    let data = read_data("input.txt")?;

    let mut start = None;
    let mut end = None;
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            if data[row][col] == 'S' {
                start = Some((row, col));
            }

            if data[row][col] == 'E' {
                end = Some((row, col));
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let start_time = std::time::Instant::now();
    let task1 = count_cheats(&data, start, end, 100, 2);
    let duration = start_time.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start_time = std::time::Instant::now();
    let task2 = count_cheats(&data, start, end, 100, 20);
    let duration = start_time.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
