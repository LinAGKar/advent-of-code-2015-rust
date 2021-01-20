use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    const TIME: u16 = 2503;

    println!("{}", input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let velocity: u16 = words.nth(3).unwrap().parse().unwrap();
        let travel_time: u16 = words.nth(2).unwrap().parse().unwrap();
        let rest_time: u16 = words.nth(6).unwrap().parse().unwrap();

        let cycle_time = travel_time + rest_time;
        let cycles = TIME / cycle_time;
        let remainder = TIME % cycle_time;
        (std::cmp::min(remainder, travel_time) + cycles * travel_time) * velocity
    }).max().unwrap());
}
