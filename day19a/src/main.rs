use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    let mut joined_input: Vec<u16> = Vec::new();
    let mut last_was_capital = false;

    for i in input {
        if (i as char).is_ascii_lowercase() && last_was_capital {
            *joined_input.last_mut().unwrap() |= (i as u16) << 8;
        } else {
            joined_input.push(i as u16);
        }
        last_was_capital = (i as char).is_ascii_uppercase();
    }
    let mut lines = joined_input.split(|&x| x == '\n' as u16);

    let mut subs = Vec::new();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let mut words = line.split(|&x| x == ' ' as u16);
        let orig = words.next().unwrap()[0];
        if orig != 'e' as u16 {
            let sub = words.nth(1).unwrap();
            subs.push((orig, sub));
        }
    }

    let orig_line = lines.next().unwrap();

    let dups_left: Vec<_> = subs.iter().cloned().filter(|&(orig, sub)| sub.starts_with(&[orig])).map(|(orig, sub)| {
        let mut count = HashMap::new();
        for &i in sub.iter().skip(1) {
            *count.entry(i).or_insert(0u8) += 1;
        }
        (orig, sub, count)
    }).collect();

    let dups_right: Vec<_> = subs.iter().cloned().filter(|&(orig, sub)| sub.ends_with(&[orig])).map(|(orig, sub)| {
        let mut count = HashMap::new();
        for &i in sub.iter().take(sub.len() - 1) {
            *count.entry(i).or_insert(0u8) += 1;
        }
        (orig, sub, count)
    }).collect();

    let dup_pairs: Vec<_> = dups_left.into_iter().filter_map(|left_sub| {
        let right_subs: Vec<_> = dups_right.iter().cloned().filter_map(|right_sub| {
            if left_sub.2 == right_sub.2 {
                Some((right_sub.0, &right_sub.1[..right_sub.1.len() - 1]))
            } else {
                None
            }
        }).collect();
        if right_subs.len() > 0 {
            Some((left_sub.0, &left_sub.1[1..], right_subs))
        } else {
            None
        }
    }).collect();

    let mut dups = std::collections::HashSet::new();
    let mut molecules = 0;

    for (n, &atom) in orig_line.iter().enumerate() {
        for (orig, sub, right_subs) in &dup_pairs {
            if *orig != atom {
                continue;
            }

            for (m, &right_atom) in orig_line.iter().enumerate().skip(n + 1) {
                let between = &orig_line[n + 1..m];

                if between.len() > sub.len() {
                    if between[..sub.len()] != **sub {
                        break;
                    }

                    if between[sub.len()..] != between[..between.len() - sub.len()] {
                        continue;
                    }
                } else if *between != sub[..between.len()] {
                    break;
                }

                for &(right_orig, right_sub) in right_subs {
                    if right_orig != right_atom {
                        continue;
                    }

                    if between.len() >= right_sub.len() {
                        if between[between.len() - right_sub.len()..] != *right_sub {
                            continue
                        }
                    } else if *between != right_sub[right_sub.len() - between.len()..] ||
                              sub[between.len()..] != right_sub[..right_sub.len() - between.len()] {
                        continue
                    }

                    dups.insert((m, right_orig, right_sub));
                }
            }
        }

        molecules += subs.iter().filter(|&&(orig, _)| orig == atom).count();
    }

    println!("{}", molecules - dups.len());
}
