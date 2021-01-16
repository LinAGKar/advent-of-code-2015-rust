use std::io::Read;

use serde_json::Value;

fn sum(value: &Value) -> i32 {
    if let Some(object) = value.as_object() {
        if object.values().any(|val| val == "red") {
            0
        } else {
            object.values().map(|val| sum(val)).sum()
        }
    } else if let Some(array) = value.as_array() {
        array.iter().map(|val| sum(val)).sum()
    } else if let Some(number) = value.as_i64() {
        number as i32
    } else {
        0
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", sum(&serde_json::from_str(&input).unwrap()))
}
