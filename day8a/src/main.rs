use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (tot_lit_len, tot_string_len) = input.lines().map(|line| {
        let mut chars = line.chars();
        let mut count = 0;
        while let Some(character) = chars.next() {
            if character == '\\' {
                if chars.next().unwrap() == 'x' {
                    chars.next();
                    chars.next();
                }
            }
            count += 1;
        }
        (line.chars().count(), count - 2)
    }).fold((0, 0), |(tot_lit_len, tot_string_len), (lit_len, string_len)| {
        (tot_lit_len + lit_len, tot_string_len + string_len)
    });
    println!("{}", tot_lit_len - tot_string_len);
}
