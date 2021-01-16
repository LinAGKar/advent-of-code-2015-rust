use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input
        .split(|character: char| !character.is_ascii_digit() && character != '-')
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<i32>().unwrap())
        .sum::<i32>());
}
