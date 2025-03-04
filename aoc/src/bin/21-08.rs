puzzle_lib::setup!(title = "Seven Segment Search");

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
 * e: 0268 (4*)
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
 * - VI: The wire that appears 4 times is segment E.
 * - VII: The wire that appears 9 times is segment F.
 *
 * Building on these we can figure out the rest of the wires as well:
 *
 * - VIII: The wire that appears in digit 1 that doesn't correspond to segment F is segment C.
 * - IX: The wire that appears in digit 4 that doesn't correspond to segments B, C, or F is segment D.
 * - X: The wire that appears in digit 7 that doesn't correspond to segments C or E is segment A.
 * - XI: The remaining wire is segment G.
 *
 * Case IV, while easy to detect, is not actually used in figuring out which wire is which segment.
 */

type Signals<'a> = [&'a str; 10];
type Digits<'a> = [&'a str; 4];
type OrderedSignals<'a> = [&'a str; 10];
type Line<'a> = (Signals<'a>, Digits<'a>);

const CHAR_OFFSET: usize = 'a' as usize;

fn parse_input(input: &str) -> Vec<Line> {
    parse!(input => {
        [lines split on '\n' with
            { [signals split] " | " [digits split] }
            => (signals.try_into().unwrap(), digits.try_into().unwrap())
        ]
    } => lines)
}

fn chr_to_idx(chr: char) -> usize {
    (chr as usize) - CHAR_OFFSET
}

fn idx_to_chr(idx: usize) -> char {
    (idx + CHAR_OFFSET) as u8 as char
}

// The strings we get are not sorted (e.g. we could get 7 as any of [acf, afc, cfa, caf, fac, fca]).
// This method converts a string into a number that is based only on the contained characters, not their order.
fn str_to_id(string: &str) -> u32 {
    string.chars().map(|c| 2u32.pow(chr_to_idx(c) as u32)).sum()
}

fn get_signal_char_used_x_times(signal_chars_uses: [u32; 7], count: u32) -> char {
    signal_chars_uses
        .into_iter()
        .enumerate()
        .find(|p| p.1 == count)
        .map(|p| idx_to_chr(p.0))
        .unwrap()
}

fn get_signal_char_not_yet_used(string: &str, used: &[char]) -> char {
    string
        .chars()
        .find(|chr| !used.contains(chr))
        .unwrap()
        .to_owned()
}

fn get_signal_char_not_using<'a>(signals: &Vec<&'a str>, chr: char) -> &'a str {
    signals.iter().find(|signal| !signal.contains(chr)).unwrap()
}

fn find_signal_mapping(signals: Signals) -> OrderedSignals {
    let mut signal_chars_uses = [0; 7];
    for chr in signals.into_iter().flat_map(str::chars) {
        signal_chars_uses[chr_to_idx(chr)] += 1;
    }

    let signal_for_1 = signals.iter().find(|signal| signal.len() == 2).unwrap();
    let signal_for_4 = signals.iter().find(|signal| signal.len() == 4).unwrap();
    let signal_for_7 = signals.iter().find(|signal| signal.len() == 3).unwrap();
    let signal_for_8 = signals.iter().find(|signal| signal.len() == 7).unwrap();

    let seg_b = get_signal_char_used_x_times(signal_chars_uses, 6);
    let seg_e = get_signal_char_used_x_times(signal_chars_uses, 4);
    let seg_f = get_signal_char_used_x_times(signal_chars_uses, 9);
    let seg_c = get_signal_char_not_yet_used(signal_for_1, &[seg_b, seg_e, seg_f]);
    // The rest of the segments aren't actually use below, so no need to actually figure them out.

    let signals_len_5 = signals
        .into_iter()
        .filter(|signal| signal.len() == 5)
        .collect::<Vec<&str>>();
    let signal_for_2 = get_signal_char_not_using(&signals_len_5, seg_f);
    let signal_for_5 = get_signal_char_not_using(&signals_len_5, seg_c);
    let signal_for_3 = signals_len_5
        .iter()
        .find(|s| s.contains(seg_c) && s.contains(seg_f))
        .unwrap();

    let signals_len_6 = signals
        .into_iter()
        .filter(|signal| signal.len() == 6)
        .collect::<Vec<&str>>();
    let signal_for_6 = get_signal_char_not_using(&signals_len_6, seg_c);
    let signal_for_9 = get_signal_char_not_using(&signals_len_6, seg_e);
    let signal_for_0 = signals_len_6
        .iter()
        .find(|s| s.contains(seg_c) && s.contains(seg_e))
        .unwrap();

    [
        signal_for_0,
        signal_for_1,
        signal_for_2,
        signal_for_3,
        signal_for_4,
        signal_for_5,
        signal_for_6,
        signal_for_7,
        signal_for_8,
        signal_for_9,
    ]
}

fn calculate_line_number(line: &Line) -> u32 {
    let mapping_as_ids: [u32; 10] = find_signal_mapping(line.0)
        .into_iter()
        .map(str_to_id)
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();
    let digits_as_ids: [u32; 4] = line
        .1
        .into_iter()
        .map(str_to_id)
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();
    [
        1000 * mapping_as_ids
            .into_iter()
            .position(|d| d == digits_as_ids[0])
            .unwrap(),
        100 * mapping_as_ids
            .into_iter()
            .position(|d| d == digits_as_ids[1])
            .unwrap(),
        10 * mapping_as_ids
            .into_iter()
            .position(|d| d == digits_as_ids[2])
            .unwrap(),
        mapping_as_ids
            .into_iter()
            .position(|d| d == digits_as_ids[3])
            .unwrap(),
    ]
    .iter()
    .sum::<usize>() as u32
}

pub fn part1(input: &str) -> u32 {
    let lines = parse_input(input);
    lines
        .iter()
        .flat_map(|line| line.1)
        .map(str::len)
        .filter(|len| [2, 3, 4, 7].contains(len))
        .count() as u32
}

pub fn part2(input: &str) -> u32 {
    let lines = parse_input(input);
    lines.iter().map(calculate_line_number).sum::<u32>() as u32
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 26, part2 = 61_229)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
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
    fn example_find_signal_mapping() {
        let actual = find_signal_mapping([
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ]);
        let expected = [
            "cagedb", "ab", "gcdfa", "fbcad", "eafb", "cdfbe", "cdfgeb", "dab", "acedgfb", "cefabd",
        ];
        assert_eq!(actual, expected);
    }
}
