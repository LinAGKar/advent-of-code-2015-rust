use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    while !(
        input.windows(3).any(|x| {
            x.windows(2).all(|y| y[1] == y[0] + 1)
        }) &&
        input
            .windows(2)
            .enumerate()
            .filter_map(|(n, x)| if x[0] == x[1] { Some(n) } else { None })
            .try_fold(None, |acc, n| {
                if let Some(start) = acc {
                    if n > start + 1 {
                        Err(())
                    } else {
                        Ok(acc)
                    }
                } else {
                    Ok(Some(n))
                }
            })
            .is_err()
    ) {
        for i in input.iter_mut().rev() {
            loop {
                *i += 1;
                if !(*i == 'i' as u8 || *i == 'o' as u8 || *i == 'l' as u8) {
                    break;
                }
            }

            if *i > 'z' as u8 {
                *i = 'a' as u8;
            } else {
                break;
            }
        }
    }

    for i in input {
        print!("{}", i as char);
    }
    println!("");
}
