use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (_, houses): (_, HashSet<_>) = input
        .chars()
        .enumerate()
        .fold((vec![(0, 0); 2], [(0, 0)].iter().cloned().collect()), |(mut positions, mut houses), (n, cmd)| {
            let pos = &mut positions[n % 2];
            match cmd {
                '^' => { pos.0 += 1; },
                'v' => { pos.0 -= 1; },
                '>' => { pos.1 += 1; },
                '<' => { pos.1 -= 1; },
                _ => {},
            };
            houses.insert(*pos);
            (positions, houses)
        });

    println!("{}", houses.len());
}
