use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Element {
    Wall,
    Empty,
    Start,
    Target,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn read_data(path: &str) -> std::io::Result<Vec<Vec<Element>>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Element::Empty,
                    '#' => Element::Wall,
                    'S' => Element::Start,
                    'E' => Element::Target,
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect())
}

const TURN_COST: i64 = 1000;

fn solve(field: &Vec<Vec<Element>>, is_part_2: bool) -> i64 {
    let start = (field.len() - 2, 1);
    let target = (1, field[0].len() - 2);

    let mut visited: Vec<Vec<[(i64, Vec<(usize, usize, Direction)>); 4]>> =
        vec![vec![std::array::from_fn(|_| (i64::MAX, Vec::new())); field[0].len()]; field.len()];

    let mut heap = BinaryHeap::new();
    heap.push((
        Reverse(0),
        start.0,
        start.1,
        Direction::Right,
        (start.0, 0, Direction::Right),
    ));

    while let Some((Reverse(cost), x, y, dir, src)) = heap.pop() {
        if (x, y) == target {
            if !is_part_2 {
                return cost;
            }

            // Collect all the possible solution
            visited[x][y][dir as usize].1.push(src);
            while let Some((Reverse(cost2), x2, y2, dir, src)) = heap.pop() {
                if cost2 > cost || (x2, y2) != target {
                    break;
                }
                visited[x][y][dir as usize].1.push(src);
            }

            // Collect all paths
            let mut queue = VecDeque::new();
            let mut visited_tiles = HashSet::new();
            visited_tiles.insert(target);

            for dir in 0..4 {
                for &(x, y, dir) in &visited[target.0][target.1][dir].1 {
                    queue.push_back((x, y, dir));
                    visited_tiles.insert((x, y));
                }
            }

            while let Some((x, y, dir)) = queue.pop_front() {
                visited_tiles.insert((x, y));
                if (x, y) == start {
                    break;
                }
                queue.extend(visited[x][y][dir as usize].1.iter().map(|&src| src));
            }

            return visited_tiles.len() as i64;
        }

        if cost > visited[x][y][dir as usize].0 {
            continue;
        }
        if cost == visited[x][y][dir as usize].0 {
            visited[x][y][dir as usize].1.push(src);
            continue;
        }
        visited[x][y][dir as usize] = (cost, vec![src]);

        let (x2, y2, next_dirs) = match dir {
            Direction::Down => (
                x + 1,
                y,
                [
                    (Direction::Left, 1),
                    (Direction::Right, 1),
                    (Direction::Up, 2),
                ],
            ),
            Direction::Left => (
                x,
                y - 1,
                [
                    (Direction::Up, 1),
                    (Direction::Down, 1),
                    (Direction::Right, 2),
                ],
            ),
            Direction::Right => (
                x,
                y + 1,
                [
                    (Direction::Up, 1),
                    (Direction::Down, 1),
                    (Direction::Left, 2),
                ],
            ),
            Direction::Up => (
                x - 1,
                y,
                [
                    (Direction::Left, 1),
                    (Direction::Right, 1),
                    (Direction::Down, 2),
                ],
            ),
        };

        if field[x2][y2] != Element::Wall {
            let total_cost = cost + 1;
            if total_cost <= visited[x2][y2][dir as usize].0 {
                heap.push((Reverse(total_cost), x2, y2, dir, (x, y, dir)));
            }
        }

        for (next_dir, turns) in next_dirs {
            let total_cost = cost + turns * TURN_COST;
            if total_cost <= visited[x][y][next_dir as usize].0 {
                heap.push((Reverse(total_cost), x, y, next_dir, (x, y, dir)));
            }
        }
    }

    -1
}

const PART_1: bool = false;
const PART_2: bool = true;

fn main() -> std::io::Result<()> {
    // let field = read_data("example.txt")?;
    // let field = read_data("example2.txt")?;
    let field = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = solve(&field, PART_1);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = solve(&field, PART_2);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
