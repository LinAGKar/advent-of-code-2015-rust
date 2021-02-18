fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let limit: u32 = input.parse().unwrap();

    let mut start = 1;
    let mut next = 1;
    let mut elves = Vec::new();
    let mut house_sums = Vec::new();
    let mut dead_elves = 0;

    'outer: loop {
        house_sums.truncate(0);
        house_sums.resize(next - start + 1, 0);

        while elves.len() < next {
            elves.push((elves.len() as u32 + 1, 0u8));
        }

        for ((pos, age), number) in elves[dead_elves..].iter_mut().zip(dead_elves as u32 + 1..) {
            while let Some(house) = house_sums.get_mut(*pos as usize - start) {
                if *age >= 50 {
                    dead_elves = number as usize;
                    break;
                }
                *house += number;
                *pos += number;
                *age += 1;
            }
        }

        for (n, sum) in house_sums.iter().enumerate() {
            if sum * 11 >= limit {
                println!("{}", start + n);
                break 'outer;
            }
        }

        start = next + 1;
        next *= 2;
    }
}
