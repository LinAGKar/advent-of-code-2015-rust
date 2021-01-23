use std::io::Read;

const AMOUNT: u8 = 150;

fn calc_combos(containers: &Vec<u8>, total: u8, pos: usize, count: u8) -> (u8, u32) {
    if total > AMOUNT {
        (u8::MAX, 0)
    } else if pos < containers.len() {
        let (a_count, a_combos) = calc_combos(containers, total, pos + 1, count);
        let (b_count, b_combos) = calc_combos(containers, total + containers[pos], pos + 1, count + 1);
        if a_count < b_count {
            (a_count, a_combos)
        } else if b_count < a_count {
            (b_count, b_combos)
        } else {
            (a_count, a_combos + b_combos)
        }
    } else if total == AMOUNT {
        (count, 1)
    } else {
        (u8::MAX, 0)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let containers: Vec<u8> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("{}", calc_combos(&containers, 0, 0, 0).1);
}
