use std::fs::read_to_string;

fn read_input(path: &str) -> std::io::Result<Vec<Vec<char>>> {
    let data = read_to_string(path)?;
    Ok(data.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect())
}

fn main() -> std::io::Result<()> {
    // let data = read_input("example.txt")?;
    let data = read_input("input.txt")?;


    let mut task1 = 0;

    let mas = vec!['M', 'A', 'S'];
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] != 'X' {
                continue;
            }

            for offset_path in [
                [(0,1),(0,2),(0,3)], // right
                [(1,1),(2,2),(3,3)], // bottom-right
                [(1,0),(2,0),(3,0)], // down
                [(1,-1),(2,-2),(3,-3)], // bottom-left
                [(0,-1),(0,-2),(0,-3)], // left
                [(-1,-1),(-2,-2),(-3,-3)], // top-left
                [(-1,0),(-2,0),(-3,0)], // top
                [(-1,1),(-2,2),(-3,3)], // top-right
            ] {
                let i = row as i32;
                let j = col as i32;

                let mut path = Vec::new();
                for (ox, oy) in offset_path {
                    let ii = i + ox;
                    let jj = j + oy;

                    if 0 <= ii && ii < (data.len() as i32) && 0 <= jj && jj < (data[0].len() as i32) {
                        path.push(data[ii as usize][jj as usize]);
                    } else {
                        break;
                    }
                }

                if path == mas {
                    task1 += 1;
                }
            }
        }
    }
    println!("Task 1: {task1}");

    let mut task2 = 0;
    let ms = vec!['M', 'S'];
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] != 'A' {
                continue;
            }

            let mut paths = Vec::new();
            for offset_path in [
                [(1,-1),(-1,1)],
                [(-1,-1),(1,1)]
            ] {
                let i = row as i32;
                let j = col as i32;

                let mut path = Vec::new();
                for (ox, oy) in offset_path {
                    let ii = i + ox;
                    let jj = j + oy;

                    if 0 <= ii && ii < (data.len() as i32) && 0 <= jj && jj < (data[0].len() as i32) {
                        path.push(data[ii as usize][jj as usize]);
                    }
                }

                path.sort_unstable();
                paths.push(path);
            }

            if paths.len() == 2 && paths[0] == ms && paths[1] == ms {
                task2 += 1;
            }
        }
    }
    println!("Task 2: {task2}");

    Ok(())
}
