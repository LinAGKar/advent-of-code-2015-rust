use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

fn calc_route<'a>(
    graph: &HashMap<u8, HashMap<u8, u16>>,
    used: &mut HashSet<u8>,
    current_pos: u8,
    current_cost: u16,
    shortest: u16,
) -> u16 {
    graph.get(&current_pos).unwrap().iter().fold(shortest, |acc, (new_pos, &cost)| {
        let new_cost = current_cost + cost;
        if new_cost >= acc || used.contains(new_pos) {
            acc
        } else if used.len() == graph.len() - 2 {
            new_cost
        } else {
            used.insert(*new_pos);
            let new_shortest = calc_route(graph, used, *new_pos, new_cost, acc);
            used.remove(new_pos);
            new_shortest
        }
    })
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut place_ids = HashMap::new();
    let mut graph = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let next_id = place_ids.len() as u8;
        let left = *place_ids.entry(words.next().unwrap()).or_insert(next_id);
        let next_id = place_ids.len() as u8;
        let right = *place_ids.entry(words.nth(1).unwrap()).or_insert(next_id);
        let cost: u16 = words.nth(1).unwrap().parse().unwrap();
        graph.entry(left).or_insert(HashMap::new()).insert(right, cost);
        graph.entry(right).or_insert(HashMap::new()).insert(left, cost);
        graph.entry(u8::MAX).or_insert(HashMap::new()).insert(right, 0);
    }

    println!("{}", calc_route(&graph, &mut HashSet::new(), u8::MAX, 0, u16::MAX));
}
