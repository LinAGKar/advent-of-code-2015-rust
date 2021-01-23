use std::io::Read;

const AMOUNT: u8 = 150;

fn calc_combos(containers: &Vec<u8>, total: u8, pos: usize) -> u32 {
    if total > AMOUNT {
        0
    } else if pos < containers.len() {
        calc_combos(containers, total, pos + 1) + calc_combos(containers, total + containers[pos], pos + 1)
    } else if total == AMOUNT {
        1
    } else {
        0
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let containers: Vec<u8> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("{}", calc_combos(&containers, 0, 0));
}
