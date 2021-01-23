use std::io::Read;

fn calc(
    ingredients: &Vec<Vec<i8>>,
    pos: usize,
    totals: &Vec<i16>,
    remaining: u8,
) -> i32 {
    if pos == ingredients.len() - 1 {
        totals.iter().zip(ingredients[pos].iter()).map(|(&a, &b)| {
            std::cmp::max(a as i16 + b as i16 * remaining as i16, 0) as i32
        }).product()
    } else {
        (0..=remaining).map(|n| {
            let new_totals = totals.iter().zip(ingredients[pos].iter()).map(|(&a, &b)| {
                a + b as i16 * n as i16
            }).collect();
            calc(ingredients, pos + 1, &new_totals, remaining - n)
        }).max().unwrap()
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let ingredients: Vec<Vec<i8>> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        [2, 1, 1, 1].iter().map(|&n| words.nth(n).unwrap().trim_matches(',').parse().unwrap()).collect()
    }).collect();

    println!("{}", calc(&ingredients, 0, &mut vec![0; ingredients[0].len()], 100));
}
