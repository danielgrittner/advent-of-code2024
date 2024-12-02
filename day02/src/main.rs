use std::fs::read_to_string;

fn read_input(path: &str) -> std::io::Result<Vec<Vec<i32>>> {
    let data = read_to_string(path)?;
    let rows = data
        .split('\n')
        .map(|row| {
            row.split(' ')
                .map(|cell| cell.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Ok(rows)
}

fn is_valid(row: &Vec<i32>, x1: usize, x2: usize, increase: bool) -> bool {
    let diff = (row[x1] - row[x2]).abs();
    1 <= diff && diff <= 3 && ((increase && row[x1] < row[x2]) || (!increase && row[x1] > row[x2]))
}

fn main() -> std::io::Result<()> {
    // let data = read_input("example.txt")?;
    let data = read_input("input.txt")?;

    let task1 = data
        .iter()
        .map(|row| {
            let increase = row[0] < row[1];

            let cnt = row.iter()
                .zip(row.iter().skip(1))
                .filter(|(&x1, &x2)| {
                    let diff = (x1 - x2).abs();
                    1 <= diff && diff <= 3 && ((increase && x1 < x2) || (!increase && x1 > x2))
                })
                .count();

            (cnt == row.len() - 1) as u8 as i32
        })
        .sum::<i32>();

    println!("Task 1: {task1}");

    let task2 = data
        .iter()
        .map(|row| {
            for increase in [true, false] {
                let mut skipped = false;
                let mut prev = vec![0];
                let mut is_violation = false;

                for idx in 1..row.len() {
                    let mut is_valid_pair = false;
                    for &p in &prev {
                        if is_valid(row, p, idx, increase) {
                            // no violation
                            is_valid_pair = true;
                            break;
                        }
                    }

                    if is_valid_pair {
                        prev.clear();
                        prev.push(idx);
                        continue;
                    }

                    if skipped {
                        is_violation = true;
                        break;
                    }

                    // check whether it is actually possible to skip prev[0], i.e., prev[0]-1 and idx is valid
                    if prev[0] == 0 || is_valid(row, prev[0]-1, idx, increase) {
                        prev = vec![prev[0], idx];
                    }
                    // else: nope, prev[0]-1 amd idx are not valid

                    skipped = true;
                }

                if !is_violation {
                    return 1;
                }
            }

            0
        })
        .sum::<i32>();

    println!("Task 2: {task2}");
    
    Ok(())
}
