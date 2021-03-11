fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut words = input.split_whitespace();
    let row: u32 = words.nth(15).unwrap().trim_matches(',').parse().unwrap();
    let column: u32 = words.nth(1).unwrap().trim_matches('.').parse().unwrap();

    let top_column = column - 1 + (row - 1);
    let sum = (top_column.pow(2) + top_column) / 2;
    let code_number = sum + column;
    println!("{}", (1..code_number).fold(20151125u64, |acc, _| {
        acc * 252533 % 33554393
    }));
}
