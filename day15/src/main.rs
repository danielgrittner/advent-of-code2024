use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Element {
    Robot,
    Box,
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Move {
    Left,
    Up,
    Right,
    Down,
}

fn read_data(path: &str) -> std::io::Result<(Vec<Vec<Element>>, Vec<Move>)> {
    let data = read_to_string(path)?;

    let mut field = Vec::new();
    let mut iter = data.split("\n");
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }

        field.push(
            line.chars()
                .map(|c| match c {
                    '#' => Element::Wall,
                    '@' => Element::Robot,
                    '.' => Element::Empty,
                    'O' => Element::Box,
                    _ => panic!("Invalid field element"),
                })
                .collect::<Vec<Element>>(),
        )
    }

    let mut moves = Vec::new();
    while let Some(line) = iter.next() {
        moves.extend(line.chars().map(|c| match c {
            '<' => Move::Left,
            '^' => Move::Up,
            '>' => Move::Right,
            'v' => Move::Down,
            _ => panic!("Invalid move"),
        }));
    }

    Ok((field, moves))
}

fn solve(mut field: Vec<Vec<Element>>, moves: &Vec<Move>) -> usize {
    let mut robot_pos_opt = None;
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == Element::Robot {
                robot_pos_opt = Some((row, col));
                break;
            }
        }

        if robot_pos_opt.is_some() {
            break;
        }
    }

    let (mut x, mut y) = robot_pos_opt.unwrap();

    let move_robot = |field: &mut Vec<Vec<Element>>,
                      old_x: usize,
                      old_y: usize,
                      new_x: usize,
                      new_y: usize|
     -> (usize, usize) {
        field[old_x][old_y] = Element::Empty;
        field[new_x][new_y] = Element::Robot;
        (new_x, new_y)
    };

    let move_box =
        |field: &mut Vec<Vec<Element>>, old_x: usize, old_y: usize, new_x: usize, new_y: usize| {
            field[old_x][old_y] = Element::Empty;
            field[new_x][new_y] = Element::Box;
        };

    for &robot_move in moves {
        let (ox, oy) = match robot_move {
            Move::Left => (0, -1),
            Move::Right => (0, 1),
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
        };

        let (x2, y2) = ((x as i32 + ox) as usize, (y as i32 + oy) as usize);

        match field[x2][y2] {
            Element::Robot => panic!("Multiple robots detected!"),
            Element::Wall => {}
            Element::Empty => {
                (x, y) = move_robot(&mut field, x, y, x2, y2);
            }
            Element::Box => {
                let (mut x3, mut y3) = (x2, y2);
                while field[x3][y3] == Element::Box {
                    (x3, y3) = ((x3 as i32 + ox) as usize, (y3 as i32 + oy) as usize);
                }

                if field[x3][y3] == Element::Empty {
                    move_box(&mut field, x2, y2, x3, y3);
                    (x, y) = move_robot(&mut field, x, y, x2, y2);
                }
            }
            Element::BoxLeft | Element::BoxRight => panic!("Unreachable"),
        }
    }

    field
        .into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, ele)| *ele == Element::Box)
                .map(|(col_idx, _)| row_idx * 100 + col_idx)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn solve2(init_field: &Vec<Vec<Element>>, moves: &Vec<Move>) -> usize {
    let mut field = vec![vec![Element::Empty; init_field[0].len() * 2]; init_field.len()];
    let mut robot_pos_opt = None;

    for row in 0..init_field.len() {
        for col in 0..init_field[row].len() {
            if init_field[row][col] == Element::Robot {
                robot_pos_opt = Some((row, 2 * col));
                field[row][2 * col] = Element::Robot;
            }

            if init_field[row][col] == Element::Wall {
                field[row][2 * col] = Element::Wall;
                field[row][2 * col + 1] = Element::Wall;
            }

            if init_field[row][col] == Element::Box {
                field[row][2 * col] = Element::BoxLeft;
                field[row][2 * col + 1] = Element::BoxRight;
            }
        }
    }

    let (mut x, mut y) = robot_pos_opt.unwrap();

    let move_robot = |field: &mut Vec<Vec<Element>>,
                      old_x: usize,
                      old_y: usize,
                      new_x: usize,
                      new_y: usize|
     -> (usize, usize) {
        field[old_x][old_y] = Element::Empty;
        field[new_x][new_y] = Element::Robot;
        (new_x, new_y)
    };

    for &robot_move in moves {
        let (ox, oy) = match robot_move {
            Move::Left => (0, -1),
            Move::Right => (0, 1),
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
        };

        let (x2, y2) = ((x as i32 + ox) as usize, (y as i32 + oy) as usize);

        match field[x2][y2] {
            Element::Robot => panic!("Multiple robots detected!"),
            Element::Wall => {}
            Element::Empty => {
                (x, y) = move_robot(&mut field, x, y, x2, y2);
            }
            Element::Box => panic!("unreachable"),
            Element::BoxLeft | Element::BoxRight => {
                match robot_move {
                    Move::Left | Move::Right => {
                        // Can we push one to the left/right?
                        let mut y3 = y2;
                        while field[x][y3] == Element::BoxLeft || field[x][y3] == Element::BoxRight
                        {
                            y3 = (y3 as i32 + 2 * oy) as usize;
                        }

                        if field[x][y3] == Element::Empty {
                            while y3 != y2 {
                                let y4 = (y3 as i32 - oy) as usize;
                                field[x][y3] = field[x][y4];
                                y3 = y4;
                            }
                            (x, y) = move_robot(&mut field, x, y, x2, y2);
                        }
                    }
                    Move::Down | Move::Up => {
                        let mut is_movable = true;

                        let mut boxes = Vec::new();
                        let mut visited = HashSet::new();

                        if field[x2][y2] == Element::BoxLeft {
                            visited.insert((x2, y2, y2 + 1));
                            boxes.push((x2, y2, y2 + 1));
                        } else {
                            visited.insert((x2, y2 - 1, y2));
                            boxes.push((x2, y2 - 1, y2));
                        }

                        let mut next_idx = 0;
                        while next_idx < boxes.len() {
                            let (x_box, y_left, y_right) = boxes[next_idx];

                            let x_box2 = (x_box as i32 + ox) as usize;

                            if field[x_box2][y_left] == Element::Wall
                                || field[x_box2][y_right] == Element::Wall
                            {
                                is_movable = false;
                                break;
                            }

                            if field[x_box2][y_left] == Element::BoxLeft {
                                let next = (x_box2, y_left, y_right);
                                if !visited.contains(&next) {
                                    visited.insert(next);
                                    boxes.push(next);
                                }
                            }

                            if field[x_box2][y_left] == Element::BoxRight {
                                let next = (x_box2, y_left - 1, y_left);
                                if !visited.contains(&next) {
                                    visited.insert(next);
                                    boxes.push(next);
                                }
                            }

                            if field[x_box2][y_right] == Element::BoxLeft {
                                let next = (x_box2, y_right, y_right + 1);
                                if !visited.contains(&next) {
                                    visited.insert(next);
                                    boxes.push(next);
                                }
                            }

                            next_idx += 1;
                        }

                        if is_movable {
                            for (source_x, y_left, y_right) in boxes.into_iter().rev() {
                                let target_x = (source_x as i32 + ox) as usize;
                                field[target_x][y_left] = Element::BoxLeft;
                                field[source_x][y_left] = Element::Empty;
                                field[target_x][y_right] = Element::BoxRight;
                                field[source_x][y_right] = Element::Empty;
                            }
                            (x, y) = move_robot(&mut field, x, y, x2, y2);
                        }
                    }
                }
            }
        }

        // println!("Move: {:?}", robot_move);
        // for row in 0..field.len() {
        //     for col in 0..field[row].len() {
        //         match field[row][col] {
        //             Element::Box => {},
        //             Element::BoxLeft => print!("["),
        //             Element::BoxRight => print!("]"),
        //             Element::Robot => print!("@"),
        //             Element::Empty => print!("."),
        //             Element::Wall => print!("#"),
        //         }
        //     }
        //     println!("");
        // }
    }

    field
        .into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, ele)| *ele == Element::BoxLeft)
                .map(|(col_idx, _)| row_idx * 100 + col_idx)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() -> std::io::Result<()> {
    // let (field, moves) = read_data("example2.txt")?;
    // let (field, moves) = read_data("example.txt")?;
    // let (field, moves) = read_data("example3.txt")?;
    let (field, moves) = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = solve(field.clone(), &moves);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = solve2(&field, &moves);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
