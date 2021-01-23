use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let constraints: std::collections::HashMap<&str, (u8, u8)> = [
        ("children", (3, 3)),
        ("cats", (8, u8::MAX)),
        ("samoyeds", (2, 2)),
        ("pomeranians", (0, 2)),
        ("akitas", (0, 0)),
        ("vizslas", (0, 0)),
        ("goldfish", (0, 4)),
        ("trees", (4, u8::MAX)),
        ("cars", (2, 2)),
        ("perfumes", (1, 1)),
    ].iter().cloned().collect();

    println!("{}", input.lines().find_map(|line| {
        let mut words = line.split_whitespace();
        let number = words.nth(1).unwrap();
        let words: Vec<_> = words.collect();
        if words.chunks(2).any(|x| {
            let (min, max) = constraints[x[0].trim_matches(':')];
            let count: u8 = x[1].trim_matches(',').parse().unwrap();
            count < min || count > max
        }) {
            None
        } else {
            Some(number.trim_matches(':').parse::<u16>().unwrap())
        }
    }).unwrap());
}
