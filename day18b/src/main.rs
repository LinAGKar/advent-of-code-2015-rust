use std::io::Read;

struct Light {
    enabled_neighboors: u8,
    timestamp: u8,
    enabled: bool,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|character| {
            Light {
                enabled_neighboors: 0,
                timestamp: u8::MAX,
                enabled: character == '#',
            }
        }).collect()
    }).collect();

    let bottom = grid.len() - 1;
    let right = grid[0].len() - 1;

    let mut enabled: Vec<_> = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, character)| {
            if character == '#' {
                Some((x, y))
            } else {
                None
            }
        })
    }).collect();

    for i in 0..100 {
        let mut touched = Vec::new();

        for (x, y) in enabled {
            for dx in if x > 0 { 0 } else { 1 }..=if x < right { 2 } else { 1 } {
                for dy in if y > 0 { 0 } else { 1 }..=if y < bottom { 2 } else { 1 } {
                    let (x, y) = (x + dx - 1, y + dy - 1);

                    let tile = &mut grid[y][x];
                    if tile.timestamp != i {
                        tile.timestamp = i;
                        tile.enabled_neighboors = 0;
                        touched.push((x, y));
                    }

                    if (dx, dy) != (1, 1) {
                        tile.enabled_neighboors += 1;
                    }
                }
            }
        }

        enabled = touched.into_iter().filter(|&(x, y)| {
            let tile = &mut grid[y][x];

            let enabled = if [0, right].contains(&x) && [0, bottom].contains(&y) {
                true
            } else if tile.enabled {
                [2, 3].contains(&tile.enabled_neighboors)
            } else {
                tile.enabled_neighboors == 3
            };
            tile.enabled = enabled;
            enabled
        }).collect();
    }

    println!("{}", enabled.len());
}
