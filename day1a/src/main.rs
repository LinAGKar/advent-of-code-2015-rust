use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.chars().fold(0, |acc, x| match x { '(' => acc + 1, ')' => acc - 1, _ => acc }));
}
