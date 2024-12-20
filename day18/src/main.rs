use std::fs::read_to_string;

fn read_data(path: &str) -> std::io::Result<Vec<Vec<usize>>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| {
            row.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect())
}

const EMPTY: u8 = 0;
const BYTE: u8 = 1;
const VISITED: u8 = 2;

fn find_path(bytes: &Vec<Vec<usize>>, h: usize, w: usize, n_bytes: usize) -> usize {
    let mut field = vec![vec![EMPTY; w]; h];
    for idx in 0..n_bytes {
        field[bytes[idx][0]][bytes[idx][1]] = BYTE;
    }

    let mut queue = vec![(0usize, 0usize)];
    field[0][0] = VISITED;
    let mut steps = 0;

    while !queue.is_empty() {
        let mut next = Vec::new();

        for (x, y) in queue {
            if (x, y) == (h - 1, w - 1) {
                return steps;
            }

            for (ox, oy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let x2 = x as i32 + ox;
                let y2 = y as i32 + oy;

                if 0 <= x2
                    && x2 < (h as i32)
                    && 0 <= y2
                    && y2 < (w as i32)
                    && field[x2 as usize][y2 as usize] == EMPTY
                {
                    next.push((x2 as usize, y2 as usize));
                    field[x2 as usize][y2 as usize] = VISITED;
                }
            }
        }

        queue = next;
        steps += 1;
    }

    usize::MAX
}

fn main() -> std::io::Result<()> {
    // let (bytes, h, w, n_bytes) = (read_data("example.txt")?, 7 as usize, 7 as usize, 12);
    let (bytes, h, w, n_bytes) = (read_data("input.txt")?, 71usize, 71usize, 1024);

    let start = std::time::Instant::now();
    let task1 = find_path(&bytes, h, w, n_bytes);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let idxs = (0..=bytes.len()).collect::<Vec<usize>>();
    let target_idx = idxs.partition_point(|&n_bytes| find_path(&bytes, h, w, n_bytes) < usize::MAX);
    let task2 = format!("{},{}", bytes[target_idx - 1][0], bytes[target_idx - 1][1]);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
