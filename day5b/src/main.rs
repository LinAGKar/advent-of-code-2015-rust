use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    println!("{}", input.split(|&x| x as u32 == '\n' as u32).filter(|&line| {
        line.windows(2).enumerate().any(|(n, x)| line[n + 2..].windows(2).any(|y| x == y)) &&
        line.iter().zip(line.iter().skip(2)).any(|(x, y)| x == y)
    }).count());
}
