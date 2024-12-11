use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<Vec<i32>>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| row.chars().map(|c| (c as u8 - '0' as u8) as i32).collect())
        .collect())
}

fn dfs_impl(map: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut Vec<Vec<i32>>) {
    visited[x][y] += 1;

    let h = map[x][y];
    if h == 9 {
        return;
    }

    for (ox, oy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let x2 = x as i32 + ox;
        let y2 = y as i32 + oy;
        if 0 <= x2 && x2 < (map.len() as i32) && 0 <= y2 && y2 < (map[0].len() as i32) {
            let x2 = x2 as usize;
            let y2 = y2 as usize;

            let h2 = map[x2][y2];

            if h + 1 == h2 {
                dfs_impl(map, x2, y2, visited);
            }
        }
    }
}

fn dfs(map: &Vec<Vec<i32>>, x: usize, y: usize, is_part2: bool) -> i32 {
    let mut visited = vec![vec![0; map[0].len()]; map.len()];
    visited[x][y] = 1;
    dfs_impl(map, x, y, &mut visited);
    if is_part2 {
        visited
            .into_iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.into_iter()
                    .enumerate()
                    .filter(|(col_idx, _)| map[row_idx][*col_idx] == 9)
                    .map(|(_, cnt)| cnt)
                    .sum::<i32>()
            })
            .sum::<i32>()
    } else {
        visited
            .into_iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.into_iter()
                    .enumerate()
                    .filter(|(col_idx, cnt)| map[row_idx][*col_idx] == 9 && *cnt > 0)
                    .count() as i32
            })
            .sum::<i32>()
    }
}

const IS_PART_1: bool = false;
const IS_PART_2: bool = true;

fn main() -> std::io::Result<()> {
    // let map = read_data("example.txt")?;
    // let map = read_data("example2.txt")?;
    let map = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let mut task1 = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                task1 += dfs(&map, row, col, IS_PART_1);
            }
        }
    }
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let mut task2 = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                task2 += dfs(&map, row, col, IS_PART_2);
            }
        }
    }
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
