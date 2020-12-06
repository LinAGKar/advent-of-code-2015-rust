use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+)x(\d+)x(\d+)$").unwrap();
    println!("{}", re.captures_iter(&input).map(|x| {
        let (l, w, h): (u32, u32, u32) = (x[1].parse().unwrap(), x[2].parse().unwrap(), x[3].parse().unwrap());
        let sides = vec![l * w, w * h, l * h];
        2 * sides.iter().sum::<u32>() + sides.iter().min().unwrap()
    }).sum::<u32>());
}
