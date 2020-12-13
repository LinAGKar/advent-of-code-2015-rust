use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (_, _, houses): (_, _, HashSet<_>) = input
        .chars()
        .fold((0, 0, [(0, 0)].iter().cloned().collect()), |(mut x, mut y, mut houses), cmd| {
            match cmd {
                '^' => { y += 1; },
                'v' => { y -= 1; },
                '>' => { x += 1; },
                '<' => { x -= 1; },
                _ => {},
            };
            houses.insert((x, y));
            (x, y, houses)
        });

    println!("{}", houses.len());
}
