use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.chars().enumerate().try_fold(0, |acc, (n, x)| match x {
        '(' => Ok(acc + 1),
        ')' => if acc > 0 { Ok(acc - 1) } else { Err(n + 1) },
        _ => Ok(acc),
    }).unwrap_err());
}
