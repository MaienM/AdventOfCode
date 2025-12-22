puzzle_runner::register_chapter!(title = "RPG Simulator 20X");

use itertools::iproduct;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Item {
    name: &'static str,
    cost: u16,
    damage: u16,
    armor: u16,
}

const ITEM_NONE: Item = Item {
    name: "None",
    cost: 0,
    damage: 0,
    armor: 0,
};
static WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
static ARMORS: [Item; 6] = [
    ITEM_NONE,
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];
static RINGS: [Item; 8] = [
    ITEM_NONE,
    ITEM_NONE,
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Debug, Eq, PartialEq)]
struct Entity {
    hp: u16,
    damage: u16,
    armor: u16,
}

fn parse_input(input: &str) -> Entity {
    parse!(input => {
        "Hit Points: " [hp as u16]
        "\nDamage: " [damage as u16]
        "\nArmor: " [armor as u16]
    } => Entity { hp, damage, armor })
}

#[inline]
fn is_win(player: &Entity, boss: &Entity) -> bool {
    let player_effective_damage = u16::max(1, player.damage.saturating_sub(boss.armor));
    let boss_effective_damage = u16::max(1, boss.damage.saturating_sub(player.armor));
    (player.hp.div_ceil(boss_effective_damage)) >= (boss.hp.div_ceil(player_effective_damage))
}

#[register_part(arg = 100)]
fn part1(input: &str, hp: u16) -> u16 {
    let boss = parse_input(input);
    let options = iproduct!(
        WEAPONS.iter(),
        ARMORS.iter(),
        RINGS.iter().tuple_combinations()
    );
    options
        .map(|(w, a, (r1, r2))| {
            let cost = w.cost + a.cost + r1.cost + r2.cost;
            let entity = Entity {
                hp,
                damage: w.damage + a.damage + r1.damage + r2.damage,
                armor: w.armor + a.armor + r1.armor + r2.armor,
            };
            (cost, entity)
        })
        .sort_by_key(|(c, _)| *c)
        .find(|(_, e)| is_win(e, &boss))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 65, part1::arg = 8)]
    static EXAMPLE_INPUT: &str = "
        Hit Points: 12
        Damage: 7
        Armor: 2
    ";

    #[test]
    fn example_is_win() {
        assert!(is_win(
            &Entity {
                hp: 8,
                damage: 5,
                armor: 5
            },
            &Entity {
                hp: 12,
                damage: 7,
                armor: 2
            }
        ));
    }
}
