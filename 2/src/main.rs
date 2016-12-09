use std::cmp;
use std::ops::Range;

fn clamp(x: i32, r: Range<i32>) -> i32 {
    cmp::min(r.end - 1, cmp::max(x, r.start))
}

fn range(x: i32) -> Range<i32> {
    let size = 2 - (2 - x).abs();
    2-size..2+size+1
}

fn main() {
    let input = include_str!("../input.txt");
    let keypad = [["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"]];
    let keypad2 = [[" ", " ", "D", " ", " "], [" ", "A", "B", "C", " "], ["5", "6", "7", "8", "9"], [" ", "2", "3", "4", " "], [" ", " ", "1", " ", " "]];

    let mut x = 1i32;
    let mut y = 1i32;
    for line in input.lines() {
        for dir in line.trim().chars() {
            match dir {
                'U' => y = clamp(y + 1, 0..3),
                'D' => y = clamp(y - 1, 0..3),
                'L' => x = clamp(x - 1, 0..3),
                'R' => x = clamp(x + 1, 0..3),
                _ => unreachable!(),
            }
        }
        print!("{}", keypad[y as usize][x as usize])
    }
    println!("");

    x = 0;
    y = 2;
    for line in input.lines() {
        for dir in line.trim().chars() {
            match dir {
                'U' => y = clamp(y + 1, range(x)),
                'D' => y = clamp(y - 1, range(x)),
                'L' => x = clamp(x - 1, range(y)),
                'R' => x = clamp(x + 1, range(y)),
                _ => unreachable!(),
            }
        }
        print!("{}", keypad2[y as usize][x as usize]);
    }
    println!("");
}
