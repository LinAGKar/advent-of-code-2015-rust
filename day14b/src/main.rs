use std::io::Read;

#[derive(Debug)]
struct Reindeer {
    velocity: u16,
    travel_time: u16,
    cycle_time: u16,
    points: u16,
    pos: u16,
    in_cycle: u16,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    const TIME: u16 = 2503;

    let mut reindeer: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let velocity = words.nth(3).unwrap().parse().unwrap();
        let travel_time = words.nth(2).unwrap().parse().unwrap();
        let rest_time: u16 = words.nth(6).unwrap().parse().unwrap();

        Reindeer {
            velocity: velocity,
            travel_time: travel_time,
            cycle_time: travel_time + rest_time,
            points: 0,
            pos: 0,
            in_cycle: 0,
        }
    }).collect();

    for _ in 0..TIME {
        let mut leader_pos = 0;
        let mut leaders = Vec::new();

        for i in &mut reindeer {
            i.in_cycle += 1;
            if i.in_cycle <= i.travel_time {
                i.pos += i.velocity;
            } else if i.in_cycle >= i.cycle_time {
                i.in_cycle = 0;
            }

            if i.pos > leader_pos {
                leader_pos = i.pos;
                leaders.clear();
            }

            if i.pos == leader_pos {
                leaders.push(i);
            }
        }

        for i in leaders {
            i.points += 1;
        }
    }

    println!("{}", reindeer.into_iter().map(|x| x.points).max().unwrap());
}
