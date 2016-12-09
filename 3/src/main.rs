fn main() {
    let input = include_str!("../input.txt");

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];

    let mut valid = 0;
    for line in input.lines() {
        let mut sides = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        a.push(sides[0]);
        b.push(sides[1]);
        c.push(sides[2]);

        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            valid += 1;
        }
    }
    println!("{}", valid);

    valid = 0;
    a.extend(b);
    a.extend(c);
    for sides in a.chunks_mut(3) {
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            valid += 1;
        }
    }
    println!("{}", valid);
}
