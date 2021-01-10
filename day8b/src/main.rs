use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (tot_lit_len, tot_string_len) = input.lines().map(|line| {
        (line.chars().map(|character| {
            match character {
                '\\' | '"' => 2,
                _ => 1,
            }
        }).sum::<usize>() + 2, line.chars().count())
    }).fold((0, 0), |(tot_lit_len, tot_string_len), (lit_len, string_len)| {
        (tot_lit_len + lit_len, tot_string_len + string_len)
    });
    println!("{}", tot_lit_len - tot_string_len);
}
