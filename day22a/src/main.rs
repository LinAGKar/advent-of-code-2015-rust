use std::cmp::max;
use std::cmp::min;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::Read;

#[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash, Debug)]
struct GameState {
    boss_hp: i8,
    hero_hp: i8,
    mana: i16,
    status_effects: u16,
}

impl GameState {
    fn neighbors(self, boss_dmg: i8) -> Neighbors {
        Neighbors {
            state: self,
            spell: 0,
            boss_dmg: boss_dmg,
        }
    }
}

struct Neighbors {
    state: GameState,
    spell: u8,
    boss_dmg: i8,
}

const SPELLS: &[(i16, &dyn Fn(GameState) -> Option<GameState>)] = &[
    (53, &|mut state: GameState| {
        state.boss_hp -= 4;
        Some(state)
    }),
    (73, &|mut state: GameState| {
        state.boss_hp -= 2;
        state.hero_hp += 2;
        Some(state)
    }),
    (113, &|mut state: GameState| {
        if state.status_effects & 0b111 != 0 {
            None
        } else {
            state.status_effects |= 6;
            Some(state)
        }
    }),
    (173, &|mut state: GameState| {
        if (state.status_effects >> 3) & 0b111 != 0 {
            None
        } else {
            state.status_effects |= 6 << 3;
            Some(state)
        }
    }),
    (229, &|mut state: GameState| {
        if (state.status_effects >> 6) & 0b111 != 0 {
            None
        } else {
            state.status_effects |= 5 << 6;
            Some(state)
        }
    }),
];

impl std::iter::Iterator for Neighbors {
    type Item = (i16, GameState);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((spell, cost, mut state)) = SPELLS.iter().enumerate().skip(self.spell as usize).find_map(|(n, &(cost, callback))| {
            let state = self.state;
            if cost > state.mana {
                None
            } else if let Some(mut state) = callback(state) {
                let shield_counter = state.status_effects & 0b111;
                let poison_counter = (state.status_effects >> 3) & 0b111;
                let recharge_counter = (state.status_effects >> 6) & 0b111;

                if poison_counter > 0 {
                    state.boss_hp -= 3; 
                }

                if state.boss_hp <= 0 {
                    return Some((n, cost, state))
                }

                state.hero_hp -= if shield_counter >= 1 {
                    max(self.boss_dmg - 7, 1)
                } else {
                    self.boss_dmg
                };

                if state.hero_hp <= 0 {
                    return None;
                }

                if poison_counter > 1 {
                    state.boss_hp -= 3; 
                }

                state.mana += min(2, recharge_counter as i16) * 101;

                let shield_counter = if shield_counter > 2 {
                    shield_counter - 2
                } else {
                    0
                };

                let poison_counter = if poison_counter > 2 {
                    poison_counter - 2
                } else {
                    0
                };

                let recharge_counter = if recharge_counter > 2 {
                    recharge_counter - 2
                } else {
                    0
                };

                state.status_effects =
                    shield_counter |
                    (poison_counter << 3) |
                    (recharge_counter << 6);

                Some((n, cost, state))
            } else {
                None
            }
        }) {
            self.spell = spell as u8 + 1;
            state.mana -= cost;
            Some((cost, state))
        } else {
            None
        }
    }
}

fn h(mut state: GameState) -> i16 {
    let mut poison_counter = (state.status_effects >> 3) & 0b111;
    let mut cost = 0;

    while poison_counter > 0 {
        if state.boss_hp <= 0 {
            return cost;
        }
        cost += 53;
        poison_counter -= 2;
        state.boss_hp -= 10;
    }

    const MANA_PER_CYCLE: i16 = 173 + 53 * 2;
    const DMG_PER_CYCLE: i8 = 6 * 3 + 4 * 2;

    if state.boss_hp >= DMG_PER_CYCLE {
        let cycles = state.boss_hp / DMG_PER_CYCLE;
        cost += cycles as i16 * MANA_PER_CYCLE;
        state.boss_hp -= cycles * DMG_PER_CYCLE;
    }

    if state.boss_hp > 20 {
        cost += MANA_PER_CYCLE;
    } else if state.boss_hp > 0 {
        cost += ((state.boss_hp - 1) / 4 + 1) as i16 * 53;
    }

    cost
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut numbers = input
        .lines()
        .map(|line| line.split_whitespace().find_map(|word| word.parse::<i8>().ok()).unwrap());
    let boss_hp = numbers.next().unwrap();
    let boss_dmg = numbers.next().unwrap();
    const HERO_HP: i8 = 50;
    const HERO_MANA: i16 = 500;

    let start = GameState {
        boss_hp: boss_hp,
        hero_hp: HERO_HP,
        mana: HERO_MANA,
        status_effects: 0,
    };

    let mut open_set = BTreeSet::new();
    open_set.insert((h(start), start));

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, h(start));

    while let Some(&state) = open_set.iter().next() {
        open_set.remove(&state);
        let (f_score, state) = state;

        if state.boss_hp <= 0 {
            println!("{}", f_score);
            break;
        }

        if state.hero_hp <= 0 {
            continue;
        }

        for (cost, neighbor) in state.neighbors(boss_dmg) {
            let tentative_g_score = g_scores[&state] + cost;
            let old_g_score = g_scores.get(&neighbor).cloned().unwrap_or(i16::MAX);
            if tentative_g_score < old_g_score {
                g_scores.insert(neighbor, tentative_g_score);
                let new_f_score = tentative_g_score + h(neighbor);
                if let Some(&old_f_score) = f_scores.get(&neighbor) {
                    open_set.remove(&(old_f_score, neighbor));
                }
                f_scores.insert(neighbor, new_f_score);
                open_set.insert((new_f_score, neighbor));
            }
        }
    }
}
