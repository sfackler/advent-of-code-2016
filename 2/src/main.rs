use std::cmp;
use std::ops::Range;

fn clamp(x: i32, r: Range<i32>) -> i32 {
    cmp::min(r.end - 1, cmp::max(x, r.start))
}

fn main() {
    let input = include_str!("../input.txt");
    let keypad = [["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"]];

    for line in input.lines() {
        let mut x = 1i32;
        let mut y = 1i32;
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
}
