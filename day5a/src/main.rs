use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.lines().filter(|&line| {
        line.chars().filter(|&x| "aeiou".contains(x)).count() >= 3 &&
        line.chars().zip(line.chars().skip(1)).any(|(x, y)| x == y) &&
        ![
            "ab",
            "cd",
            "pq",
            "xy",
        ].iter().any(|x| line.contains(x))
    }).count());
}
