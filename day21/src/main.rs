use std::collections::HashMap;
use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<Vec<char>>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| row.chars().collect())
        .collect())
}

const DIRECTIONAL_KEYPAD: [char; 5] = ['<', 'v', '>', '^', 'A'];
const NUMERICAL_KEYPAD: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
const EMPTY_KEY: char = '\n';

trait Grid {
    fn at(&self, row: usize, col: usize) -> char;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

impl<const R: usize, const C: usize> Grid for [[char; C]; R] {
    fn at(&self, row: usize, col: usize) -> char {
        self.get(row).unwrap().get(col).copied().unwrap()
    }

    fn height(&self) -> usize {
        R
    }

    fn width(&self) -> usize {
        C
    }
}

fn generate_shortest_paths_impl<T: Grid>(
    current: (usize, usize),
    target: char,
    grid: &T,
    visited: &mut Vec<Vec<bool>>,
    path: &mut Vec<char>,
    results: &mut Vec<Vec<char>>,
) {
    if grid.at(current.0, current.1) == target {
        results.push(path.clone());
        return;
    }

    for (ox, oy, key) in [(1, 0, 'v'), (-1, 0, '^'), (0, 1, '>'), (0, -1, '<')] {
        let x = current.0 as i32 + ox;
        let y = current.1 as i32 + oy;
        if 0 <= x && x < (grid.height() as i32) && 0 <= y && y < (grid.width() as i32) {
            let (x, y) = (x as usize, y as usize);

            if grid.at(x, y) != EMPTY_KEY && !visited[x][y] {
                visited[x][y] = true;
                path.push(key);

                generate_shortest_paths_impl((x, y), target, grid, visited, path, results);

                path.pop();
                visited[x][y] = false;
            }
        }
    }
}

fn generate_shortest_paths<T: Grid>(start: char, target: char, grid: &T) -> Vec<Vec<char>> {
    let mut start_pos = (usize::MAX, usize::MAX);
    for idx1 in 0..grid.height() {
        for idx2 in 0..grid.width() {
            if grid.at(idx1, idx2) == start {
                start_pos = (idx1, idx2);
            }
        }
    }

    let mut visited = vec![vec![false; grid.width()]; grid.height()];
    let mut results = Vec::new();
    let mut path = Vec::new();

    generate_shortest_paths_impl(
        start_pos,
        target,
        grid,
        &mut visited,
        &mut path,
        &mut results,
    );

    // only keep shortest paths
    let shortest_length = results
        .iter()
        .map(|result| result.len())
        .min()
        .expect("should have a shortest length");
    results
        .into_iter()
        .filter(|result| result.len() == shortest_length)
        .collect()
}

fn compute_shortest_paths() -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut shortest_paths = HashMap::new();

    let dir_keypad: [[char; 3]; 2] = [[EMPTY_KEY, '^', 'A'], ['<', 'v', '>']];

    for idx in 0..DIRECTIONAL_KEYPAD.len() {
        for idx2 in 0..DIRECTIONAL_KEYPAD.len() {
            let (from, to) = (DIRECTIONAL_KEYPAD[idx], DIRECTIONAL_KEYPAD[idx2]);

            let shortest_paths_from_to = generate_shortest_paths(from, to, &dir_keypad);
            shortest_paths.insert((from, to), shortest_paths_from_to);
        }
    }

    let num_keypad: [[char; 3]; 4] = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [EMPTY_KEY, '0', 'A'],
    ];

    for idx in 0..NUMERICAL_KEYPAD.len() {
        for idx2 in 0..NUMERICAL_KEYPAD.len() {
            let (from, to) = (NUMERICAL_KEYPAD[idx], NUMERICAL_KEYPAD[idx2]);

            let shortests_paths_from_to = generate_shortest_paths(from, to, &num_keypad);
            shortest_paths.insert((from, to), shortests_paths_from_to);
        }
    }

    shortest_paths
}

fn find_shortest_path(
    memo: &mut HashMap<(usize, Vec<char>), usize>,
    layer: usize,
    shortest_paths: &HashMap<(char, char), Vec<Vec<char>>>,
    current_path: &Vec<char>,
    intermediate_layers: usize,
) -> usize {
    if layer == intermediate_layers {
        return current_path.len();
    }

    let memo_key = (layer, current_path.clone());
    if let Some(&result) = memo.get(&memo_key) {
        return result;
    }

    let mut length = 0;

    let mut layer_pos = 'A';
    for idx in 0..current_path.len() {
        let transition = (layer_pos, current_path[idx]);

        let mut sub_result = usize::MAX;
        for subpath in shortest_paths.get(&transition).unwrap() {
            let mut path = subpath.clone();
            path.push('A');
            sub_result = std::cmp::min(
                sub_result,
                find_shortest_path(memo, layer + 1, shortest_paths, &path, intermediate_layers),
            );
        }

        length += sub_result;
        layer_pos = current_path[idx];
    }

    memo.insert(memo_key, length);

    length
}

fn to_digit(c: char) -> usize {
    (c as u8 - '0' as u8) as usize
}

fn main() -> std::io::Result<()> {
    // let data = read_data("example.txt")?;
    let data = read_data("input.txt")?;

    let shortest_paths = compute_shortest_paths();

    let start_time = std::time::Instant::now();
    let mut task1 = 0;
    for path in &data {
        let mut memo = HashMap::new();
        let shortest_path_length = find_shortest_path(&mut memo, 0, &shortest_paths, &path, 3);

        task1 += shortest_path_length
            * (100 * to_digit(path[0]) + 10 * to_digit(path[1]) + to_digit(path[2]));
    }
    let duration = start_time.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start_time = std::time::Instant::now();
    let mut task2 = 0;
    for path in &data {
        let mut memo = HashMap::new();
        let shortest_path_length = find_shortest_path(&mut memo, 0, &shortest_paths, &path, 26);

        task2 += shortest_path_length
            * (100 * to_digit(path[0]) + 10 * to_digit(path[1]) + to_digit(path[2]));
    }
    let duration = start_time.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
