//! Solving this for a generic input for the provided problem space is not computationally
//! feasible. The trick here is that the input follows some specific rules which allow us to
//! reduce the problem space.
//!
//! The first important observation is that the instructions are essentially 14 mini-programs
//! that only consider a single-digit input & the accumulated value for `z` (the input is read
//! into `w`, and both `x` and `y` are reset to 0 with `mul ? 0` before being used).
//!
//! The second is that these smaller programs are pretty fomulaic. In fact, if we reduce each of
//! these programs into rust code we have only two forms, as displayed below. `w` is the input
//! digit and `a` and `b` are constants in the range `-15..=15`.
//!
//! ```rust
//! if (z % 26) + a != w {
//!    z = z * 26 + w + b
//! }
//! ```
//!
//! ```rust
//! if (z % 26) + a != w {
//!    z = z / 26
//!    z = z * 26 + w + b
//! } else {
//!    z = z / 26
//! }
//! ```
//!
//! The first form can only ever leave `z` as-is or increase it (theoretically there are some
//! conditions where this isn't true, but this doesn't seem to ever occur in practice and
//! wouldn't be by a meaningful amount anyway). Note that it's not always possible to provide a
//! value for `w` where the conditional will not run.
//!
//! The second form _can_ meaningfully decrease `z`, but only if the conditional code isn't run. In
//! fact, if we can avoid running this conditional code an instance of the second form cancels out
//! an instance of the first form (`((z * 26) + w + b) / 26` == `z` as long as `w + b` is in the
//! range `0..26`, which it will be in most cases).
//!
//! The third observation is that this effectively functions as a stack, so if we have (in order)
//! forms `1 1 2 2` then the third will cancel out the result of the second, and the fourth the
//! result of the first. We can't predict fully predict what combinations of inputs will make a
//! pair cancel each other out, but we _can_ figure out the relative values of these inputs.
//!
//! Lets look at a pair and figure out how we can calculate this. After the first step
//! `z = z * 26 + w1 + b1`. To avoid triggering the conditional in the second step we need
//! `(z * 26 + w1 + b1) % 26 + a2 == w2`. `(z * 26) % 26` will always be zero, so we can simplify
//! this to `(w1 + b1) % 26 + a2 == w2`. Given that `w1` is in range `1..=9` and `b1` is in range
//! `-15..=15` we can eliminate this modulo (the combined range `-14..24` doesn't contain any
//! values that will be wrapped by it) and further simplify this to `w1 + b1 + a2 == w2`, which we
//! can use to figure out the possible sets of inputs for a given pair.
//!
//! For example lets take a pair with `a1 = 10`, `b1 = 8`, `a2 = -11` and `b2 = 12`. Filling in
//! these values into the formula gives us `w1 + 8 - 11 = w2` which we can simplify to `w1 - 3 =
//! w2`. This means that the following pairs of inputs could work: `41`, `52`, `63`, `74`, `85`,
//! `96`.
//!
//! This doesn't take into consideration the possibility that the conditional in form 1 will be
//! skipped. If this is the case then the conditional in form 2 will not be skipped, which means
//! that the net result will be a change in `z`. We're not going to bother being clever about
//! this, the above observations yield a small enough problem space that we can simply run the
//! instructions for the candidate inputs until we get one that works.

use std::ops::Range;

puzzle_lib::setup!(title = "Arithmetic Logic Unit");

type Memory = [isize; 4];

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Registry {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}
impl TryFrom<&str> for Registry {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Registry::W),
            "x" => Ok(Registry::X),
            "y" => Ok(Registry::Y),
            "z" => Ok(Registry::Z),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Rhs {
    Reg(Registry),
    Num(isize),
}
impl From<&str> for Rhs {
    fn from(value: &str) -> Self {
        if let Ok(registry) = Registry::try_from(value) {
            Rhs::Reg(registry)
        } else {
            Rhs::Num(value.parse().unwrap())
        }
    }
}
impl Rhs {
    fn get(&self, memory: &Memory) -> isize {
        match self {
            Rhs::Reg(registry) => memory[*registry as usize],
            Rhs::Num(num) => *num,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruction {
    Inp(Registry),
    Add(Registry, Rhs),
    Mul(Registry, Rhs),
    Div(Registry, Rhs),
    Mod(Registry, Rhs),
    Eql(Registry, Rhs),
}
impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut parts = value.split(' ');

        let instruction = parts.next().unwrap();
        let registry = parts.next().unwrap().try_into().unwrap();
        if instruction == "inp" {
            return Instruction::Inp(registry);
        }

        let rhs: Rhs = parts.next().unwrap().into();
        match instruction {
            "add" => Instruction::Add(registry, rhs),
            "mul" => Instruction::Mul(registry, rhs),
            "div" => Instruction::Div(registry, rhs),
            "mod" => Instruction::Mod(registry, rhs),
            "eql" => Instruction::Eql(registry, rhs),
            v => panic!("Invalid instruction {v}."),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => { [instructions split on '\n' as Instruction] } => instructions)
}

fn run(instructions: &[Instruction], memory: &mut Memory, inputs: &[isize]) {
    assert_eq!(
        instructions
            .iter()
            .filter(|i| matches!(i, Instruction::Inp(_)))
            .count(),
        inputs.len(),
        "Incorrect number of inputs.",
    );

    let mut inputs = inputs.iter().copied();
    for instruction in instructions {
        match instruction {
            Instruction::Inp(registry) => {
                let Some(value) = inputs.next() else {
                    return;
                };
                memory[*registry as usize] = value;
                // memory[*registry as usize] = inputs.next().unwrap();
            }
            Instruction::Add(registry, rhs) => {
                memory[*registry as usize] += rhs.get(memory);
            }
            Instruction::Mul(registry, rhs) => {
                memory[*registry as usize] *= rhs.get(memory);
            }
            Instruction::Div(registry, rhs) => {
                memory[*registry as usize] /= rhs.get(memory);
            }
            Instruction::Mod(registry, rhs) => {
                memory[*registry as usize] %= rhs.get(memory);
            }
            Instruction::Eql(registry, rhs) => {
                memory[*registry as usize] =
                    isize::from(memory[*registry as usize] == rhs.get(memory));
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    range_first: Range<isize>,
    offset: isize,
    indexes: [usize; 2],
}

fn get_pairs(instructions: &[Instruction]) -> Vec<Pair> {
    // Split into sets of instructions based on when a new input is read.
    let sets = instructions.iter().batching(|it| {
        let mut set = vec![it.next()?];
        set.extend(it.take_while_ref(|i| !matches!(i, Instruction::Inp(_))));
        Some(set)
    });

    // Figure out form, a, and b for each set.
    let sets = sets
        .map(|set| {
            let form = set
                .iter()
                .find_map(|i| {
                    if let Instruction::Div(Registry::Z, Rhs::Num(num)) = i {
                        // 1 => form 1, 26 => form 2
                        Some(usize::from(*num == 26))
                    } else {
                        None
                    }
                })
                .unwrap();
            let a = set
                .iter()
                .find_map(|i| {
                    if let Instruction::Add(Registry::X, Rhs::Num(num)) = i {
                        Some(num)
                    } else {
                        None
                    }
                })
                .unwrap();
            let b = set
                .iter()
                .filter_map(|i| {
                    if let Instruction::Add(Registry::Y, Rhs::Num(num)) = i {
                        Some(num)
                    } else {
                        None
                    }
                })
                .next_back()
                .unwrap();
            (form, a, b)
        })
        .collect::<Vec<_>>();
    assert_eq!(sets.len(), 14);

    // Match up the sets and figure out the contraints for the pair.
    let mut stack = Vec::new();
    let mut pairs = Vec::with_capacity(7);
    for (idx, (form, a, _)) in sets.iter().copied().enumerate() {
        if form == 0 {
            stack.push(idx);
        } else {
            let first_idx = stack.pop().unwrap();
            let (_, _, b1) = sets[first_idx];
            let offset = b1 + a;
            pairs.push(Pair {
                range_first: if offset > 0 {
                    1..(10 - offset)
                } else {
                    (1 - offset)..10
                },
                offset,
                indexes: [first_idx, idx],
            });
        }
    }

    // Sort them by their first index
    pairs.sort_by_key(|p| p.indexes[0]);

    pairs
}

pub fn part1(input: &str) -> String {
    let instructions = parse_input(input);
    let pairs = get_pairs(&instructions);

    for nums in pairs
        .iter()
        .map(|pair| pair.range_first.clone().rev())
        .multi_cartesian_product()
    {
        let mut inputs = [0; 14];
        for (i, pair) in pairs.iter().enumerate() {
            inputs[pair.indexes[0]] = nums[i];
            inputs[pair.indexes[1]] = nums[i] + pair.offset;
        }

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &inputs);
        if memory[Registry::Z as usize] == 0 {
            return inputs.iter().join("");
        }
    }
    panic!("Failed to find solution.");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT_1: &str = "
        inp x
        mul x -1
    ";

    #[example_input]
    static EXAMPLE_INPUT_2: &str = "
        inp z
        inp x
        mul z 3
        eql z x
    ";

    #[example_input]
    static EXAMPLE_INPUT_3: &str = "
        inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2
    ";

    #[example_input]
    static EXAMPLE_INPUT_4: &str = "
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 10
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 10
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -11
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 12
        mul y x
        add z y
    ";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![
            Instruction::Inp(Registry::X),
            Instruction::Mul(Registry::X, Rhs::Num(-1)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = vec![
            Instruction::Inp(Registry::Z),
            Instruction::Inp(Registry::X),
            Instruction::Mul(Registry::Z, Rhs::Num(3)),
            Instruction::Eql(Registry::Z, Rhs::Reg(Registry::X)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_3() {
        let actual = parse_input(&EXAMPLE_INPUT_3);
        let expected = vec![
            Instruction::Inp(Registry::W),
            Instruction::Add(Registry::Z, Rhs::Reg(Registry::W)),
            Instruction::Mod(Registry::Z, Rhs::Num(2)),
            Instruction::Div(Registry::W, Rhs::Num(2)),
            Instruction::Add(Registry::Y, Rhs::Reg(Registry::W)),
            Instruction::Mod(Registry::Y, Rhs::Num(2)),
            Instruction::Div(Registry::W, Rhs::Num(2)),
            Instruction::Add(Registry::X, Rhs::Reg(Registry::W)),
            Instruction::Mod(Registry::X, Rhs::Num(2)),
            Instruction::Div(Registry::W, Rhs::Num(2)),
            Instruction::Mod(Registry::W, Rhs::Num(2)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn run_example_1() {
        let instructions = parse_input(&EXAMPLE_INPUT_1);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[4]);
        assert_eq!(memory[Registry::X as usize], -4);
    }

    #[test]
    fn run_example_2() {
        let instructions = parse_input(&EXAMPLE_INPUT_2);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[4, 10]);
        assert_eq!(memory[Registry::Z as usize], 0);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[4, 12]);
        assert_eq!(memory[Registry::Z as usize], 1);
    }

    #[test]
    fn run_example_3() {
        let instructions = parse_input(&EXAMPLE_INPUT_3);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[10]);
        assert_eq!(memory, [1, 0, 1, 0]);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[7]);
        assert_eq!(memory, [0, 1, 1, 1]);
    }

    #[test]
    fn run_example_4() {
        let instructions = parse_input(&EXAMPLE_INPUT_4);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[2, 1]);
        assert_eq!(memory[Registry::Z as usize], 0);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[3, 2]);
        assert_eq!(memory[Registry::Z as usize], 0);

        let mut memory = Memory::default();
        run(&instructions, &mut memory, &[4, 3]);
        assert_eq!(memory[Registry::Z as usize], 0);
    }
}
