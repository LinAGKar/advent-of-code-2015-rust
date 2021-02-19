use std::cmp::max;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut numbers = input
        .lines()
        .map(|line| line.split_whitespace().find_map(|word| word.parse::<i8>().ok()).unwrap());

    let boss_hp = numbers.next().unwrap();
    let boss_dmg = numbers.next().unwrap();
    let boss_armor = numbers.next().unwrap();
    const HERO_HP: i8 = 100;

    let weapons = [
        ("Dagger", 8, 4, 0),
        ("Shortsword", 10, 5, 0),
        ("Warhammer", 25, 6, 0),
        ("Longsword", 40, 7, 0),
        ("Greataxe", 74, 8, 0),
    ];

    let armors = [
        ("None", 0, 0, 0),
        ("Leather", 13, 0, 1),
        ("Chainmail", 31, 0, 2),
        ("Splintmail", 53, 0, 3),
        ("Bandedmail", 75, 0, 4),
        ("Platemail", 102, 0, 5),
    ];

    let rings = [
        ("Damage +1", 25, 1, 0),
        ("Damage +2", 50, 2, 0),
        ("Damage +3", 100, 3, 0),
        ("Defense +1", 20, 0, 1),
        ("Defense +2", 40, 0, 2),
        ("Defense +3", 80, 0, 3),
    ];

    let ring_combos: Vec<_> = rings.iter().enumerate().flat_map(|(n, ring)| {
        rings
            .iter()
            .skip(n + 1)
            .map(move |other_ring| vec![ring, other_ring])
    }).chain(rings.iter().map(|ring| vec![ring])).chain([Vec::new()].iter().cloned()).collect();

    let combos = weapons.len() * armors.len() * ring_combos.len();

    println!("{}", (0..combos).filter_map(|i| {
        let weapon = &weapons[i % weapons.len()];
        let i = i / weapons.len();

        let armor = &armors[i % armors.len()];
        let i = i / armors.len();

        let ring_combo = &ring_combos[i];

        let hero_dmg = weapon.2 + ring_combo.iter().map(|ring| ring.2).sum::<i8>();
        let hero_armor = armor.3 + ring_combo.iter().map(|ring| ring.3).sum::<i8>();
        let cost = weapon.1 + armor.1 + ring_combo.iter().map(|ring| ring.1).sum::<u16>();

        let hero_lifetime = (HERO_HP - 1) / (max(1, boss_dmg - hero_armor)) + 1;
        let boss_lifetime = (boss_hp - 1) / (max(1, hero_dmg - boss_armor)) + 1;

        if hero_lifetime >= boss_lifetime {
            Some(cost)
        } else {
            None
        }
    }).min().unwrap());
}
