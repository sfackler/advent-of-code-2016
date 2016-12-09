fn main() {
    let input = include_str!("../input.txt");

    let mut valid = 0;
    for line in input.lines() {
        let mut sides = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            valid += 1;
        }
    }
    println!("{}", valid);
}
