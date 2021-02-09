use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let molecule = input.lines().skip_while(|line| line.len() > 0).nth(1).unwrap();
    let mut atoms = Vec::new();

    for letter in molecule.chars() {
        if !letter.is_ascii_lowercase() {
            atoms.push(String::new());
        }
        atoms.last_mut().unwrap().push(letter);
    }

    println!(
        "{}",
        atoms.len() -
        atoms.iter().filter(|&atom| ["Rn", "Ar"].contains(&(atom as &str))).count() -
        2 * atoms.iter().filter(|&atom| atom == "Y").count() -
        1,
    );
}
