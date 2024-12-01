use std::ops::{AddAssign, SubAssign};

use aoc::utils::parse;
use rayon::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct Cost {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

#[derive(Debug, Eq, PartialEq)]
struct Blueprint {
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    parse!(input => {
        [blueprints split on "\n" with
            {
                "Blueprint " _ ": "
                "Each ore robot costs " [ore_ore as u16] " ore. "
                "Each clay robot costs " [clay_ore as u16] " ore. "
                "Each obsidian robot costs " [obsidian_ore as u16] " ore and " [obsidian_clay as u16] " clay. "
                "Each geode robot costs " [geode_ore as u16] " ore and " [geode_obsidian as u16] " obsidian."
            }
            => Blueprint {
                ore: Cost {
                    ore: ore_ore,
                    clay: 0,
                    obsidian: 0,
                },
                clay: Cost {
                    ore: clay_ore,
                    clay: 0,
                    obsidian: 0,
                },
                obsidian: Cost {
                    ore: obsidian_ore,
                    clay: obsidian_clay,
                    obsidian: 0,
                },
                geode: Cost {
                    ore: geode_ore,
                    clay: 0,
                    obsidian: geode_obsidian,
                },
            }
        ]
    } => blueprints)
}

#[derive(Clone, Debug, Default)]
struct StateCounters {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}
impl AddAssign<&StateCounters> for StateCounters {
    fn add_assign(&mut self, rhs: &Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}
impl SubAssign<&Cost> for StateCounters {
    fn sub_assign(&mut self, rhs: &Cost) {
        *self = Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode,
        };
    }
}
impl StateCounters {
    fn can_make(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn ore(ore: u16) -> Self {
        Self {
            ore,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn clay(clay: u16) -> Self {
        Self {
            ore: 0,
            clay,
            obsidian: 0,
            geode: 0,
        }
    }

    fn obsidian(obsidian: u16) -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian,
            geode: 0,
        }
    }

    fn geode(geode: u16) -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct SkippedCrafting {
    ore: bool,
    clay: bool,
    obsidian: bool,
}
impl SkippedCrafting {
    fn clear(&mut self) {
        *self = Self {
            ore: false,
            clay: false,
            obsidian: false,
        };
    }
}

#[derive(Clone, Debug)]
struct State {
    resources: StateCounters,
    robots: StateCounters,
    target: StateCounters,
    factory: Option<StateCounters>,
    skipped: SkippedCrafting,
    cycles: u16,
}
impl State {
    fn build_robot(&mut self, cost: &Cost, result: StateCounters) {
        self.resources -= cost;
        self.factory = Option::Some(result);
    }
}

fn run_cycles(mut state: State, blueprint: &Blueprint) -> u16 {
    state.resources += &state.robots;
    state.cycles -= 1;
    if state.cycles == 0 {
        return state.resources.geode;
    }

    if let Option::Some(built) = state.factory {
        state.robots += &built;
        state.factory = Option::None;
        state.skipped.clear();
    }

    if state.resources.can_make(&blueprint.geode) {
        // If we can make a geode robot this will always be optimal, so don't even consider other paths.
        state.build_robot(&blueprint.geode, StateCounters::geode(1));
        return run_cycles(state, blueprint);
    }

    let mut results = Vec::new();

    if !state.skipped.ore
        && state.robots.ore <= state.target.ore
        && state.resources.can_make(&blueprint.ore)
    {
        state.skipped.ore = true;

        let mut state = state.clone();
        state.build_robot(&blueprint.ore, StateCounters::ore(1));
        state.skipped.clear();
        results.push(run_cycles(state, blueprint));
    }

    if !state.skipped.clay
        && state.robots.clay <= state.target.clay
        && state.robots.ore + 2 >= state.target.ore
        && state.resources.can_make(&blueprint.clay)
    {
        state.skipped.clay = true;

        let mut state = state.clone();
        state.build_robot(&blueprint.clay, StateCounters::clay(1));
        results.push(run_cycles(state, blueprint));
    }

    if !state.skipped.obsidian
        && state.robots.obsidian <= state.target.obsidian
        && state.robots.ore + 2 >= state.target.ore
        && state.robots.clay + 2 >= state.target.clay
        && state.resources.can_make(&blueprint.obsidian)
    {
        state.skipped.obsidian = true;

        let mut state = state.clone();
        state.build_robot(&blueprint.obsidian, StateCounters::obsidian(1));
        results.push(run_cycles(state, blueprint));
    }

    results.push(run_cycles(state, blueprint));

    results.into_iter().max().unwrap()
}

fn calculate_geode_production(blueprint: &Blueprint, cycles: u16) -> u16 {
    let mut targets = Vec::new();
    for ore in 1..7 {
        for clay in 2..11 {
            for obsidian in 2..11 {
                targets.push(StateCounters {
                    ore,
                    clay,
                    obsidian,
                    geode: 0,
                });
            }
        }
    }

    targets
        .into_iter()
        .map(|target| {
            run_cycles(
                State {
                    resources: StateCounters::default(),
                    robots: StateCounters::default(),
                    target,
                    factory: Option::Some(StateCounters::ore(1)),
                    skipped: SkippedCrafting::default(),
                    cycles: cycles + 1,
                },
                blueprint,
            )
        })
        .max()
        .unwrap()
}

pub fn part1(input: &str) -> u16 {
    let blueprints = parse_input(input);
    blueprints
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| (i + 1) as u16 * calculate_geode_production(blueprint, 24))
        .sum()
}

pub fn part2(input: &str) -> u16 {
    let blueprints = parse_input(input);
    blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| calculate_geode_production(blueprint, 32))
        .product()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 33, part2 = 3472, notest)]
    static EXAMPLE_INPUT: &str = "
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Blueprint {
                ore: Cost {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                },
                clay: Cost {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                },
                obsidian: Cost {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                },
                geode: Cost {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                },
            },
            Blueprint {
                ore: Cost {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                },
                clay: Cost {
                    ore: 3,
                    clay: 0,
                    obsidian: 0,
                },
                obsidian: Cost {
                    ore: 3,
                    clay: 8,
                    obsidian: 0,
                },
                geode: Cost {
                    ore: 3,
                    clay: 0,
                    obsidian: 12,
                },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[ignore = "slow"]
    #[test]
    fn example_calculate_geode_production_1_24() {
        let blueprint = Blueprint {
            ore: Cost {
                ore: 4,
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: 3,
                clay: 14,
                obsidian: 0,
            },
            geode: Cost {
                ore: 2,
                clay: 0,
                obsidian: 7,
            },
        };
        assert_eq!(calculate_geode_production(&blueprint, 24), 9);
    }

    #[ignore = "slow"]
    #[test]
    fn example_calculate_geode_production_2_24() {
        let blueprint = Blueprint {
            ore: Cost {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: 3,
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: 3,
                clay: 8,
                obsidian: 0,
            },
            geode: Cost {
                ore: 3,
                clay: 0,
                obsidian: 12,
            },
        };
        assert_eq!(calculate_geode_production(&blueprint, 24), 12);
    }

    #[ignore = "slow"]
    #[test]
    fn example_calculate_geode_production_1_32() {
        let blueprint = Blueprint {
            ore: Cost {
                ore: 4,
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: 3,
                clay: 14,
                obsidian: 0,
            },
            geode: Cost {
                ore: 2,
                clay: 0,
                obsidian: 7,
            },
        };
        assert_eq!(calculate_geode_production(&blueprint, 32), 56);
    }

    #[ignore = "slow"]
    #[test]
    fn example_calculate_geode_production_2_32() {
        let blueprint = Blueprint {
            ore: Cost {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: 3,
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: 3,
                clay: 8,
                obsidian: 0,
            },
            geode: Cost {
                ore: 3,
                clay: 0,
                obsidian: 12,
            },
        };
        assert_eq!(calculate_geode_production(&blueprint, 32), 62);
    }
}
