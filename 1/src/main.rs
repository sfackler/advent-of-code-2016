use std::collections::HashSet;
use std::cmp;

enum Direction {
    North,
    South,
    East,
    West
}

fn main() {
    let input = include_str!("../input.txt");

    let mut x = 0i32;
    let mut y = 0i32;
    let mut direction = Direction::North;
    let mut locations = HashSet::new();

    for step in input.split(", ") {
        let (rotate, count) = step.split_at(1);
        direction = match (rotate, direction) {
            ("R", Direction::North) => Direction::East,
            ("R", Direction::South) => Direction::West,
            ("R", Direction::East) => Direction::South,
            ("R", Direction::West) => Direction::North,
            ("L", Direction::North) => Direction::West,
            ("L", Direction::South) => Direction::East,
            ("L", Direction::East) => Direction::North,
            ("L", Direction::West) => Direction::South,
            _ => unreachable!(),
        };
        let count = count.trim().parse::<i32>().unwrap();

        let x_range;
        let y_range;
        match direction {
            Direction::North => {
                x_range = x..x+1;
                y_range = y+1..y+count+1;
                y += count
            }
            Direction::South => {
                x_range = x..x+1;
                y_range = y-count..y;
                y -= count
            },
            Direction::East => {
                x_range = x+1..x+count+1;
                y_range = y..y+1;
                x += count
            },
            Direction::West => {
                x_range = x-count..x;
                y_range = y..y+1;
                x -= count
            },
        }

        for x in x_range {
            for y in y_range.clone() {
                if !locations.insert((x, y)) {
                    println!("{}", x.abs() + y.abs());
                }
            }
        }

    }

    println!("{}", x.abs() + y.abs());
}
