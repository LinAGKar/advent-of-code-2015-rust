use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

fn find_max<'a>(
    start: &str,
    happinesses: &HashMap<&'a str, HashMap<&str, i16>>,
    used: &mut HashSet<&'a str>,
    curr: &str,
    happiness: i16,
) -> i16 {
    if used.len() == happinesses.len() {
        happiness + happinesses[curr][start] + happinesses[start][curr]
    } else {
        happinesses.keys().filter_map(|&name| {
            if used.contains(name) {
                None
            } else {
                used.insert(name);
                let max = find_max(
                    start, happinesses, used, name,
                    happiness + happinesses[curr][name] + happinesses[name][curr],
                );
                used.remove(name);
                Some(max)
            }
        }).max().unwrap()
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut happinesses = HashMap::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let left = words.next().unwrap();
        let direction = words.nth(1).unwrap();
        let val: i16 = words.next().unwrap().parse().unwrap();
        let val = if direction == "gain" { val } else { -val };
        let right = words.nth(6).unwrap().trim_matches('.');
        happinesses.entry(left).or_insert_with(HashMap::new).insert(right, val);
    }

    let &start = happinesses.keys().next().unwrap();
    println!("{}", find_max(start, &happinesses, &mut [start].iter().cloned().collect(), start, 0));
}
