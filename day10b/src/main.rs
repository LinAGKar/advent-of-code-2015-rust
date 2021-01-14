use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    while !input.last().unwrap().is_ascii_digit() {
        input.truncate(input.len() - 1);
    }

    for _ in 0..50 {
        let (_, _, new_seq) = input
            .into_iter()
            .chain([0].iter().cloned())
            .fold((None, 0, Vec::new()), |(prev, count, mut new_seq), character| {
                if prev.is_none() || prev.unwrap() == character {
                    (Some(character), count + 1, new_seq)
                } else {
                    new_seq.push('0' as u8 + count);
                    new_seq.push(prev.unwrap());
                    (Some(character), 1, new_seq)
                }
            });

        input = new_seq;
    }

    println!("{}", input.len());
}
