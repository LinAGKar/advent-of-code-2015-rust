fn main() {
    println!("{}", (0..).try_fold((113383u64, 0), |(a, b), _| {
        if a == 1 {
            Err(b)
        } else {
            Ok((if a % 2 == 0 {
                a / 2
            } else {
                a * 3 + 1
            }, b + 1))
        }
    }).unwrap_err());
}
