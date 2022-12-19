use aoc::runner::*;
use std::ops::{AddAssign, SubAssign};

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

fn parse_input(input: String) -> Vec<Blueprint> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.trim().split(" ");
            let ore = Cost {
                ore: parts.nth(6).unwrap().parse().unwrap(),
                clay: 0,
                obsidian: 0,
            };
            let clay = Cost {
                ore: parts.nth(5).unwrap().parse().unwrap(),
                clay: 0,
                obsidian: 0,
            };
            let obsidian = Cost {
                ore: parts.nth(5).unwrap().parse().unwrap(),
                clay: parts.nth(2).unwrap().parse().unwrap(),
                obsidian: 0,
            };
            let geode = Cost {
                ore: parts.nth(5).unwrap().parse().unwrap(),
                clay: 0,
                obsidian: parts.nth(2).unwrap().parse().unwrap(),
            };
            return Blueprint {
                ore,
                clay,
                obsidian,
                geode,
            };
        })
        .collect();
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
        return self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian;
    }

    fn ore(ore: u16) -> Self {
        return Self {
            ore,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
    }

    fn clay(clay: u16) -> Self {
        return Self {
            ore: 0,
            clay,
            obsidian: 0,
            geode: 0,
        };
    }

    fn obsidian(obsidian: u16) -> Self {
        return Self {
            ore: 0,
            clay: 0,
            obsidian,
            geode: 0,
        };
    }

    fn geode(geode: u16) -> Self {
        return Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode,
        };
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

    match state.factory {
        Option::Some(built) => {
            state.robots += &built;
            state.factory = Option::None;
            state.skipped.clear();
        }
        _ => {}
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
        results.push(run_cycles(state, &blueprint));
    }

    if !state.skipped.clay
        && state.robots.clay <= state.target.clay
        && state.resources.can_make(&blueprint.clay)
    {
        state.skipped.clay = true;

        let mut state = state.clone();
        state.build_robot(&blueprint.clay, StateCounters::clay(1));
        results.push(run_cycles(state, &blueprint));
    }

    if !state.skipped.obsidian
        && state.robots.obsidian <= state.target.obsidian
        && state.resources.can_make(&blueprint.obsidian)
    {
        state.skipped.obsidian = true;

        let mut state = state.clone();
        state.build_robot(&blueprint.obsidian, StateCounters::obsidian(1));
        results.push(run_cycles(state, &blueprint));
    }

    results.push(run_cycles(state, &blueprint));

    return results.into_iter().max().unwrap();
}

fn calculate_geode_production(blueprint: &Blueprint, cycles: u16) -> u16 {
    let mut targets = Vec::new();
    for ore in (1..8).rev() {
        for clay in (1..8).rev() {
            for obsidian in (1..8).rev() {
                targets.push(StateCounters {
                    ore,
                    clay,
                    obsidian,
                    geode: 0,
                });
            }
        }
    }

    return targets
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
        .unwrap();
}

pub fn part1(input: String) -> u16 {
    let blueprints = parse_input(input);
    let mut result = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        result += (i + 1) as u16 * calculate_geode_production(blueprint, 24);
    }
    return result;
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
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

    #[test]
    fn example_calculate_geode_production_1() {
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

    #[test]
    fn example_calculate_geode_production_2() {
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

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 33);
    }
}
