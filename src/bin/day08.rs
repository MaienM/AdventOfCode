use aoc::*;

/*
 * Overview:
 *
 *    0:      1:      2:      3:      4:
 *   aaaa    ....    aaaa    aaaa    ....
 *  b    c  .    c  .    c  .    c  b    c
 *  b    c  .    c  .    c  .    c  b    c
 *   ....    ....    dddd    dddd    dddd
 *  e    f  .    f  e    .  .    f  .    f
 *  e    f  .    f  e    .  .    f  .    f
 *   gggg    ....    gggg    gggg    ....
 *
 *    5:      6:      7:      8:      9:
 *   aaaa    aaaa    aaaa    aaaa    aaaa
 *  b    .  b    .  .    c  b    c  b    c
 *  b    .  b    .  .    c  b    c  b    c
 *   dddd    dddd    ....    dddd    dddd
 *  .    f  e    f  .    f  e    f  .    f
 *  .    f  e    f  .    f  e    f  .    f
 *   gggg    gggg    ....    gggg    gggg
 *
 * Used segments per digit, marked those with a unique amount:
 *
 * 0: abcefg (6)
 * 1: cf (2*)
 * 2: acdeg (5)
 * 3: acdfg (5)
 * 4: bcdf (4*)
 * 5: abdfg (5)
 * 6: abdefg (6)
 * 7: ace (3*)
 * 8: abcdefg (7*)
 * 9: abcdfg (6)
 *
 * Uses per segment:
 *
 * a: 02356789 (8)
 * b: 045689 (6*)
 * c: 01234789 (8)
 * d: 3456789 (7)
 * e: 02689 (5*)
 * f: 013456789 (9*)
 * g: 0235689 (7)
 *
 * From this we can formulate the following easy to detect cases:
 *
 * - I: The output with 2 wires is digit 1.
 * - II: The output with 4 wires is digit 4.
 * - III: The output with 3 wires is digit 7.
 * - IV: The output with 7 wires is digit 8.
 *
 * - V: The wire that appears 6 times is segment B.
 * - VI: The wire that appears 5 times is segment E.
 * - VII: The wire that appears 9 times is segment F.
 *
 * Building on these we can figure out the rest of the wires as well:
 *
 * - VIII: The wire that appears in digit 1 that doesn't correspond to segment F is segment C.
 * - IX: The wire that is used 8 times that doesn't correspond to segment C is segment A.
 * - X: The wire that appears in digit 4 that doesn't correspond to segments B, C, or F is segment D.
 * - XI: The remaining wire is segment G.
 *
 * Case II and IV, while easy to detect, are not actually used in figuring out which wire is which segment.
 */

type Signals<'a> = [&'a str; 10];
type Digits<'a> = [&'a str; 4];
type Line<'a> = (Signals<'a>, Digits<'a>);

fn parse_line<'a>(line: &'a str) -> Line<'a> {
    let parts: [&str; 2] = line
        .splitn(2, "|")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    let patterns: [&str; 10] = parts[0]
        .trim()
        .split(" ")
        .map(str::trim)
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    let digits: [&str; 4] = parts[1]
        .trim()
        .split(" ")
        .map(str::trim)
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    return (patterns, digits);
}

fn parse_input<'a>(input: &'a str) -> Vec<Line<'a>> {
    return input.trim().split("\n").map(parse_line).collect();
}

fn part1(input: String) -> i64 {
    let lines = parse_input(&input);
    return lines
        .iter()
        .flat_map(|line| line.1)
        .map(str::len)
        .filter(|len| [2, 3, 4, 7].contains(len))
        .count() as i64;
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = vec![
            (
                [
                    "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                    "fabcd", "edb",
                ],
                ["fdgacbe", "cefdb", "cefbgd", "gcbe"],
            ),
            (
                [
                    "edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde",
                    "gfcbed", "gfec",
                ],
                ["fcgedb", "cgb", "dgebacf", "gc"],
            ),
            (
                [
                    "fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb",
                    "cdgabef",
                ],
                ["cg", "cg", "fdcagb", "cbg"],
            ),
            (
                [
                    "fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca",
                    "fcdbega",
                ],
                ["efabcd", "cedba", "gadfec", "cb"],
            ),
            (
                [
                    "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab",
                    "fcbdga",
                ],
                ["gecf", "egdcabf", "bgf", "bfgea"],
            ),
            (
                [
                    "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                    "bafgc", "acf",
                ],
                ["gebdcfa", "ecba", "ca", "fadegcb"],
            ),
            (
                [
                    "dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb",
                    "gdcebf", "gf",
                ],
                ["cefg", "dcbef", "fcge", "gbcadfe"],
            ),
            (
                [
                    "bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg",
                    "gebcd",
                ],
                ["ed", "bcgafe", "cdgba", "cbgef"],
            ),
            (
                [
                    "egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb",
                    "bfceg",
                ],
                ["gbdfcae", "bgc", "cg", "cgb"],
            ),
            (
                [
                    "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                    "fegbdc",
                ],
                ["fgae", "cfgab", "fg", "bagce"],
            ),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 26);
    }
}
