use std::fs::read_to_string;
use regex::Regex;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

const COST_BUTTON_A: i64 = 3;
const COST_BUTTON_B: i64 = 1;

fn read_data(path: &str) -> std::io::Result<Vec<Game>> {
    let regex_button = Regex::new(r"Button [AB]: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let regex_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    let mut games = Vec::new();
    let mut next_game = Game::default();
    for line in read_to_string(path)?.split("\n") {
        if line.starts_with("Button A:") {
            let caps = regex_button.captures(line).unwrap();

            let x = caps["x"].parse::<i64>().unwrap();
            let y = caps["y"].parse::<i64>().unwrap();

            next_game.button_a = (x, y);
        } else if line.starts_with("Button B:") {
            let caps = regex_button.captures(line).unwrap();

            let x = caps["x"].parse::<i64>().unwrap();
            let y = caps["y"].parse::<i64>().unwrap();

            next_game.button_b = (x, y);
        } else if line.starts_with("Prize: ") {
            let caps = regex_prize.captures(line).unwrap();

            let x = caps["x"].parse::<i64>().unwrap();
            let y = caps["y"].parse::<i64>().unwrap();

            next_game.prize = (x, y);

            games.push(next_game);
            next_game = Game::default();
        }
    }
    Ok(games)
}

const PART_2_OFFSET: i64 = 10000000000000;

const PART_1: bool = false;
const PART_2: bool = true;

fn solve2(game: Game, is_part_2: bool) -> i64 {
    let (prize_x, prize_y) = if is_part_2 {
        (game.prize.0 + PART_2_OFFSET, game.prize.1 + PART_2_OFFSET)
    } else {
        game.prize
    };

    // Solve for [t1, t2]:
    //
    // | p1 |   | a1    b1  |   | t1 |
    // |    | = |           | * |    |
    // | p2 |   | a2    b2  |   | t2 |
    
    let mut equation_system = [
        [game.button_a.0 as f64, game.button_b.0 as f64],
        [game.button_a.1 as f64, game.button_b.1 as f64]
    ];
    let mut target = [prize_x as f64, prize_y as f64];

    // 1) II - a2/a1 * I
    if equation_system[0][0] == 0.0 {
        // No solution possible in this case.
        return 0;
    }
    let m = equation_system[1][0] / equation_system[0][0];
    equation_system[1][0] -= m * equation_system[0][0];
    equation_system[1][1] -= m * equation_system[0][1];
    target[1] -= m * target[0];

    if equation_system[1][1] == 0.0 {
        // No solution
        return 0;
    }

    // 2) t2 = p2 / (b2 - a2/a1 * b1)
    let t2 = target[1] / equation_system[1][1];

    equation_system[1][1] = 1.0;
    target[1] = t2;

    // 3) I - b1 * II
    target[0] -= equation_system[0][1] * t2;
    equation_system[0][1] = 0.0;

    // t1 = (p1 - b1 * t2) / a1
    let t1 = target[0] / equation_system[0][0];
    
    equation_system[0][0] = 1.0;
    target[0] = t1;

    for (t1, t2) in [
        (t1.ceil() as i64, t2.ceil() as i64),
        (t1.floor() as i64, t2.ceil() as i64),
        (t1.ceil() as i64, t2.floor() as i64),
        (t1.floor() as i64, t2.floor() as i64),
    ] {
        // Check correctness of result
        if game.button_a.0 * t1 + game.button_b.0 * t2 == prize_x && game.button_a.1 * t1 + game.button_b.1 * t2 == prize_y {
            return COST_BUTTON_A * t1 + COST_BUTTON_B * t2;
        }
    }

    0
}

fn main() -> std::io::Result<()> {
    // let data = read_data("example.txt")?;
    let data = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = data.iter().map(|&game| solve2(game, PART_1)).sum::<i64>();
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let task2 = data.iter().map(|&game| solve2(game, PART_2)).sum::<i64>();
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}

/*


Button A: X+94, Y+34
Button B: X+22, Y+67

Prize: X=8400, Y=5400



f(t) = [94, 34] * t
g(t) = [22, 67] * t

[8400, 5400] = f(t1) + g(t2)
[8400, 5400] = [94, 34] * t1 + [22, 67] * t2

Find all solutions:

| 8400 |    | 94    22  |   | t1 |
|      | =  |           | * |    |
| 5400 |    | 34    67  |   | t2 |


| p1 |   | a1    b1  |   | t1 |
|    | = |           | * |    |
| p2 |   | a2    b2  |   | t2 |


| a1    b1 ||  p1 |
| a2    b2 ||  p2 |

1. II - a2/a1 * I

| a1    b1              || p1  |
| 0     b2 - a2/a1 * b1 || p2  |

2. t2 = p2 / (b2 - a2/a1 * b1)

| a1    b1  || p1  |
| 0     1   || t2  |

3. I - b1 * II

| a1    0   || p1 - b1 * t2  |
| 0     1   || t2            |

4. t1 = (p1 - b1 * t2) / a1

| 1     0   || t1 |
| 0     1   || t2 |

*/