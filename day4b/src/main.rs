use std::io;
use std::io::Read;

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let orig_len = input.len();
    println!("{}", (0..).find(|&x| {
        input.truncate(orig_len);
        input.append(&mut format!("{}", x).into_bytes());
        let hash = md5::compute(&input);
        (0..3).all(|y| hash.0[y] == 0)
    }).unwrap());
}
