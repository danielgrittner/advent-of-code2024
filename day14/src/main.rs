use regex::Regex;
use std::fs::read_to_string;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

fn read_data(path: &str) -> std::io::Result<Vec<Robot>> {
    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| {
            let caps = re.captures(row).unwrap();
            Robot {
                pos: (
                    caps["px"].parse::<i32>().unwrap(),
                    caps["py"].parse::<i32>().unwrap(),
                ),
                velocity: (
                    caps["vx"].parse::<i32>().unwrap(),
                    caps["vy"].parse::<i32>().unwrap(),
                ),
            }
        })
        .collect())
}

fn quadrant_product(moved_robots: Vec<(i32, i32)>, width: i32, height: i32) -> i32 {
    let mid_width = width / 2;
    let mid_height = height / 2;

    let mut quadrants: [i32; 4] = [0, 0, 0, 0];
    for (x, y) in moved_robots {
        // 1st quadrant?
        if x < mid_width && y < mid_height {
            quadrants[0] += 1;
            continue;
        }

        // 2nd quadrant
        if x > mid_width && y < mid_height {
            quadrants[1] += 1;
            continue;
        }

        // 3rd quadrant
        if x < mid_width && y > mid_height {
            quadrants[2] += 1;
            continue;
        }

        // 4th quadrant
        if x > mid_width && y > mid_height {
            quadrants[3] += 1;
        }
    }

    quadrants.into_iter().product()
}

fn solve(
    robots: &Vec<Robot>,
    width: i32,
    height: i32,
    seconds: i32,
    print: bool,
) -> Vec<(i32, i32)> {
    assert!(seconds > 0);

    let mut field = vec![vec![0; width as usize]; height as usize];

    let mut moved_robots: Vec<(i32, i32)> = robots
        .into_iter()
        .map(|robot| {
            (
                (robot.pos.0 + robot.velocity.0).rem_euclid(width),
                (robot.pos.1 + robot.velocity.1).rem_euclid(height),
            )
        })
        .map(|(x, y)| {
            field[y as usize][x as usize] += 1;
            (x, y)
        })
        .collect();

    for _ in 2..=seconds {
        // Update robot positions
        for robot_id in 0..moved_robots.len() {
            let (x, y) = moved_robots[robot_id];
            field[y as usize][x as usize] -= 1;

            let (x, y) = (
                (x + robots[robot_id].velocity.0).rem_euclid(width),
                (y + robots[robot_id].velocity.1).rem_euclid(height),
            );
            moved_robots[robot_id] = (x, y);
            field[y as usize][x as usize] += 1;
        }
    }

    if print {
        for idx in 0..field.len() {
            for idx2 in 0..field[idx].len() {
                if field[idx][idx2] > 0 {
                    print!(" {} ", field[idx][idx2]);
                } else {
                    print!("   ");
                }
            }
            println!("");
        }
    }

    moved_robots
}

fn main() -> std::io::Result<()> {
    // let (data, width, height) = (read_data("example.txt")?, 11, 7);
    let (data, width, height) = (read_data("input.txt")?, 101, 103);

    let start = std::time::Instant::now();
    let task1 = quadrant_product(solve(&data, width, height, 100, false), width, height);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = 6532;
    solve(&data, width, height, task2, true);
    let duration = start.elapsed();
    println!("Task 2 (took {:?})", duration);

    Ok(())
}
