use std::fs::read_to_string;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Field {
    Obstacle,
    Visited,
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn read_input(path: &str) -> std::io::Result<((usize, usize), Vec<Vec<Field>>)> {
    let data = read_to_string(path)?;
    let field = data
        .split("\n")
        .map(|row| {
            row.to_string()
                .chars()
                .map(|c| {
                    if c == '.' {
                        Field::Empty
                    } else if c == '^' {
                        Field::Visited
                    } else {
                        Field::Obstacle
                    }
                })
                .collect::<Vec<Field>>()
        })
        .collect::<Vec<Vec<Field>>>();

    let mut found = false;
    let mut pos = (0, 0);
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == Field::Visited {
                found = true;
                pos = (row, col);
                break;
            }
        }

        if found {
            break;
        }
    }

    Ok((pos, field))
}

fn traverse_impl(
    mut field: Vec<Vec<Field>>,
    pos: (usize, usize),
    mut dir: Direction,
    mut visited: Vec<Vec<[bool; 4]>>,
) -> Option<Vec<Vec<Field>>> {
    let (mut x, mut y) = pos;
    loop {
        match dir {
            Direction::Up => {
                if visited[x][y][0] {
                    return None;
                }
                visited[x][y][0] = true;

                if x == 0 {
                    break;
                }

                if field[x - 1][y] == Field::Obstacle {
                    dir = Direction::Right;
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                if visited[x][y][1] {
                    return None;
                }
                visited[x][y][1] = true;

                if y + 1 == field[0].len() {
                    break;
                }

                if field[x][y + 1] == Field::Obstacle {
                    dir = Direction::Down;
                } else {
                    y += 1;
                }
            }
            Direction::Down => {
                if visited[x][y][2] {
                    return None;
                }
                visited[x][y][2] = true;

                if x + 1 == field.len() {
                    break;
                }

                if field[x + 1][y] == Field::Obstacle {
                    dir = Direction::Left;
                } else {
                    x += 1;
                }
            }
            Direction::Left => {
                if visited[x][y][3] {
                    return None;
                }
                visited[x][y][3] = true;

                if y == 0 {
                    break;
                }

                if field[x][y - 1] == Field::Obstacle {
                    dir = Direction::Up;
                } else {
                    y -= 1;
                }
            }
        }

        field[x][y] = Field::Visited;
    }

    Some(field)
}

fn traverse(field: Vec<Vec<Field>>, pos: (usize, usize)) -> Option<Vec<Vec<Field>>> {
    let visited = vec![vec![[false; 4]; field[0].len()]; field.len()];
    traverse_impl(field, pos, Direction::Up, visited)
}

fn main() -> std::io::Result<()> {
    // let (pos, mut field) = read_input("example.txt")?;
    let (pos, mut field) = read_input("input.txt")?;

    let field2 = traverse(field.clone(), pos).unwrap();
    let task1 = field2
        .iter()
        .map(|row| row.iter().filter(|&&x| x == Field::Visited).count())
        .sum::<usize>();
    println!("Task 1: {task1}");

    let mut task2 = 0;
    for row in 0..field2.len() {
        for col in 0..field2[row].len() {
            if field2[row][col] != Field::Visited || (row, col) == pos {
                continue;
            }

            field[row][col] = Field::Obstacle;
            if traverse(field.clone(), pos).is_none() {
                task2 += 1;
            }
            field[row][col] = Field::Empty;
        }
    }
    println!("Task 2: {task2}");

    Ok(())
}
