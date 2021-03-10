use std::io::Read;

fn increment(taken: &mut Vec<usize>, increase: bool, max: usize) -> bool {
    if increase {
        match taken.iter().enumerate().rev().skip(1).find(|&(n, &i)| {
            *taken.last().unwrap() - i > taken.len() - n
        }) {
            Some((index, _)) => {
                let start = taken[index] + 1;
                for (n, i) in taken.iter_mut().skip(index).enumerate() {
                    *i = start + n;
                }
                true
            }

            None => false
        }
    } else {
        match taken.iter().enumerate().rev().find(|&(n, &i)| {
            max - i > taken.len() - n
        }) {
            Some((index, _)) => {
                let start = taken[index] + 1;
                for (n, i) in taken.iter_mut().skip(index).enumerate() {
                    *i = start + n;
                }
                true
            }

            None => false
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let weights: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let weights: Vec<_> = weights.into_iter().rev().collect();
    let total: u32 = weights.iter().sum();

    for i in 0..weights.len() {
        let mut taken: Vec<_> = (0..i).collect();
        let mut best_entanglement = u64::MAX;
        loop {
            let group_total: u32 = taken.iter().map(|&x| weights[x]).sum();
            if group_total == total / 3 {
                let remaining_weights: Vec<_> = weights.iter().take(taken[0]).cloned().chain(
                    taken.windows(2).flat_map(|j| {
                        &weights[j[0] + 1..j[1]]
                    }).cloned()
                ).chain(weights.iter().skip(*taken.last().unwrap() + 1).cloned()).collect();
                if (1..=remaining_weights.len()).any(|count| {
                    let mut taken: Vec<_> = (0..count).collect();
                    loop {
                        let group_total: u32 = taken.iter().map(|&x| remaining_weights[x]).sum();
                        if group_total == total / 3 {
                            return true;
                        }
                        if !increment(&mut taken, group_total <= total / 3, remaining_weights.len()) {
                            break;
                        }
                    }
                    false
                }) {
                    let entanglement: u64 = taken.iter().map(|&x| weights[x] as u64).product();
                    if entanglement < best_entanglement {
                        best_entanglement = entanglement;
                    }
                }
            }
            if !increment(&mut taken, group_total <= total / 3, weights.len()) {
                break;
            }
        }
        if best_entanglement < u64::MAX {
            println!("{}", best_entanglement);
            break;
        }
    }
}
