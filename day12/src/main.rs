use std::collections::VecDeque;
use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<Vec<char>>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|num| num.chars().collect())
        .collect())
}

fn calculate_price(map: &Vec<Vec<char>>, is_part2: bool) -> i32 {
    let mut visited = vec![vec![-1; map[0].len()]; map.len()];
    let mut next_id = 0i32;
    let mut price = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if visited[row][col] != -1 {
                continue;
            }

            let target = map[row][col];

            let mut area = 0;
            let mut perimeter = 0;

            let mut queue = VecDeque::new();
            queue.push_back((row, col));
            visited[row][col] = next_id;

            while let Some((x, y)) = queue.pop_front() {
                area += 1;

                let mut fence_sides = 4;

                for (ox, oy) in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
                    let x2 = x as i32 + ox;
                    let y2 = y as i32 + oy;

                    if 0 <= x2
                        && x2 < (map.len() as i32)
                        && 0 <= y2
                        && y2 < (map[0].len() as i32)
                        && map[x2 as usize][y2 as usize] == target
                    {
                        fence_sides -= 1;
                        if visited[x2 as usize][y2 as usize] != next_id {
                            visited[x2 as usize][y2 as usize] = next_id;
                            queue.push_back((x2 as usize, y2 as usize));
                        }
                    }
                }

                perimeter += fence_sides;
            }

            price += area * perimeter;
            next_id += 1;
        }
    }

    if is_part2 {
        let mut side_counts = vec![0; next_id as usize];
        let mut area_counts = vec![0; next_id as usize];

        // horizontal scan
        for row in 0..map.len() {
            let mut prev = i32::MAX;
            let mut prev_top_side = false;
            let mut prev_bottom_side = false;

            for col in 0..map[row].len() {
                let id = visited[row][col];
                area_counts[id as usize] += 1;

                let top_side = row == 0 || visited[row - 1][col] != id;
                let bottom_side = row == map.len() - 1 || visited[row + 1][col] != id;

                if id != prev {
                    prev = id;
                    side_counts[id as usize] += top_side as i32;
                    side_counts[id as usize] += bottom_side as i32;
                } else {
                    side_counts[id as usize] += (!prev_top_side && top_side) as i32;
                    side_counts[id as usize] += (!prev_bottom_side && bottom_side) as i32;
                }

                prev_top_side = top_side;
                prev_bottom_side = bottom_side;
            }
        }

        // vertical scan
        for col in 0..map[0].len() {
            let mut prev = i32::MAX;
            let mut prev_left_side = false;
            let mut prev_right_side = false;

            for row in 0..map.len() {
                let id = visited[row][col];

                let left_side = col == 0 || visited[row][col - 1] != id;
                let right_side = col == map[0].len() - 1 || visited[row][col + 1] != id;

                if id != prev {
                    prev = id;
                    side_counts[id as usize] += left_side as i32;
                    side_counts[id as usize] += right_side as i32;
                } else {
                    side_counts[id as usize] += (!prev_left_side && left_side) as i32;
                    side_counts[id as usize] += (!prev_right_side && right_side) as i32;
                }

                prev_left_side = left_side;
                prev_right_side = right_side;
            }
        }

        return side_counts
            .into_iter()
            .zip(area_counts.into_iter())
            .map(|(sides, area)| sides * area)
            .sum();
    }

    price
}

const PART_1: bool = false;
const PART_2: bool = true;

fn main() -> std::io::Result<()> {
    // let map = read_data("example.txt")?;
    let map = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = calculate_price(&map, PART_1);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = calculate_price(&map, PART_2);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
