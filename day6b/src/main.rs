use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<u8>> = (0..1000).map(|_| (0..1000).map(|_| 0).collect()).collect();
    let regex = regex::Regex::new(r"(?m)^(turn (?:off|on)|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    for caps in regex.captures_iter(&input) {
        let (xa, ya, xb, yb) = (
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
            caps[5].parse::<usize>().unwrap(),
        );
        let cmd = &caps[1];
        for x in xa..=xb {
            for y in ya..=yb {
                if cmd == "turn on" {
                    grid[x][y] += 1;
                } else if cmd == "turn off" {
                    if grid[x][y] > 0 {
                        grid[x][y] -= 1;
                    }
                } else if cmd == "toggle" {
                    grid[x][y] += 2;
                }
            }
        }
    }

    println!("{}", grid.into_iter().map(|x| x.into_iter().map(|y| y as u32).sum::<u32>()).sum::<u32>());
}
