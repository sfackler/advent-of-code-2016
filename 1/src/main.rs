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
        match direction {
            Direction::North => y += count,
            Direction::South => y -= count,
            Direction::East => x += count,
            Direction::West => x -= count,
        }
    }

    println!("{}", x.abs() + y.abs());
}
