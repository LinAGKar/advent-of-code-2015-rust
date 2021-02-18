fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let limit = input.parse().unwrap();

    let mut start = 1;
    let mut next = 1;
    let mut elves = Vec::new();
    let mut house_sums = Vec::new();

    'outer: loop {
        house_sums.truncate(0);
        house_sums.resize(next - start + 1, 0);

        while elves.len() < next {
            elves.push(elves.len() as u32 + 1);
        }

        for (pos, number) in elves.iter_mut().zip(1..) {
            while let Some(house) = house_sums.get_mut(*pos as usize - start) {
                *house += number;
                *pos += number;
            }
        }

        for (n, sum) in house_sums.iter().enumerate() {
            if sum * 10 >= limit {
                println!("{}", start + n);
                break 'outer;
            }
        }

        start = next + 1;
        next *= 2;
    }
}
