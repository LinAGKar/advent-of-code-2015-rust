use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let constraints: std::collections::HashMap<&str, u8> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ].iter().cloned().collect();

    println!("{}", input.lines().find_map(|line| {
        let mut words = line.split_whitespace();
        let number = words.nth(1).unwrap();
        let words: Vec<_> = words.collect();
        if words.chunks(2).any(|x| {
            constraints[x[0].trim_matches(':')] != x[1].trim_matches(',').parse().unwrap()
        }) {
            None
        } else {
            Some(number.trim_matches(':').parse::<u16>().unwrap())
        }
    }).unwrap());
}
