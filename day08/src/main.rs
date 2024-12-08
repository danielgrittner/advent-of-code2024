use std::fs::read_to_string;
use std::collections::HashMap;

fn read_input(path: &str) -> std::io::Result<Vec<Vec<char>>> {
    let data = read_to_string(path)?;
    Ok(data.split("\n").map(|row| row.chars().collect()).collect())
}

fn count_antinodes2(field: &Vec<Vec<char>>, part1: bool) -> i32 {
    let mut clusters: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == '.' {
                continue;
            }
            clusters.entry(field[row][col])
                .and_modify(|coordinates| coordinates.push((row, col)))
                .or_insert_with(|| vec![(row, col)]);
        }
    }

    let m = field.len() as i32;
    let n = field[0].len() as i32;
    
    let mut cnt = 0;
    let mut antinodes_pos = vec![vec![false; n as usize]; m as usize];

    for (frequency, coordinates) in clusters {
        for idx1 in 0..coordinates.len() {
            for idx2 in (idx1+1)..coordinates.len() {
                let ox = coordinates[idx1].0 as i32 - coordinates[idx2].0 as i32;
                let oy = coordinates[idx1].1 as i32 - coordinates[idx2].1 as i32;

                let mut t = if part1 { 1 } else { 0 };
                loop {
                    let x = coordinates[idx1].0 as i32 + t * ox;
                    let y = coordinates[idx1].1 as i32 + t * oy;

                    if 0 <= x && x < m && 0 <= y && y < n {
                        if !antinodes_pos[x as usize][y as usize] {
                            antinodes_pos[x as usize][y as usize] = true;
                            cnt += 1;
                        }

                        t += 1;
                        if part1 {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let mut t = if part1 { 1 } else { 0 };
                loop {
                    let x = coordinates[idx2].0 as i32 - t * ox;
                    let y = coordinates[idx2].1 as i32 - t * oy;

                    if 0 <= x && x < m && 0 <= y && y < n {
                        if !antinodes_pos[x as usize][y as usize] {
                            antinodes_pos[x as usize][y as usize] = true;
                            cnt += 1;
                        }

                        t += 1;
                        if part1 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    cnt
}

fn main() -> std::io::Result<()> {
    // let data = read_input("example.txt")?;
    // let data = read_input("example2.txt")?;
    let data = read_input("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = count_antinodes2(&data, true);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = count_antinodes2(&data, false);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);


    Ok(())
}
