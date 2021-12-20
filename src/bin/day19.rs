use std::{
    collections::{BTreeSet, HashSet},
    ops::{Add, Mul, Sub},
};

use aoc::runner::*;

const MATCH_THRESHOLD: usize = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Matrix((i32, i32, i32), (i32, i32, i32), (i32, i32, i32));
impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        return Matrix(
            (
                self.0 .0 * rhs.0 .0 + self.0 .1 * rhs.1 .0 + self.0 .2 * rhs.2 .0,
                self.0 .0 * rhs.0 .1 + self.0 .1 * rhs.1 .1 + self.0 .2 * rhs.2 .1,
                self.0 .0 * rhs.0 .2 + self.0 .1 * rhs.1 .2 + self.0 .2 * rhs.2 .2,
            ),
            (
                self.1 .0 * rhs.0 .0 + self.1 .1 * rhs.1 .0 + self.1 .2 * rhs.2 .0,
                self.1 .0 * rhs.0 .1 + self.1 .1 * rhs.1 .1 + self.1 .2 * rhs.2 .1,
                self.1 .0 * rhs.0 .2 + self.1 .1 * rhs.1 .2 + self.1 .2 * rhs.2 .2,
            ),
            (
                self.2 .0 * rhs.0 .0 + self.2 .1 * rhs.1 .0 + self.2 .2 * rhs.2 .0,
                self.2 .0 * rhs.0 .1 + self.2 .1 * rhs.1 .1 + self.2 .2 * rhs.2 .1,
                self.2 .0 * rhs.0 .2 + self.2 .1 * rhs.1 .2 + self.2 .2 * rhs.2 .2,
            ),
        );
    }
}

const IDENTITY_MATRIX: Matrix = Matrix((1, 0, 0), (0, 1, 0), (0, 0, 1));
const ROTATION_MATRICES: [Matrix; 24] = [
    Matrix((1, 0, 0), (0, 1, 0), (0, 0, 1)),
    Matrix((1, 0, 0), (0, 0, 1), (0, -1, 0)),
    Matrix((1, 0, 0), (0, 0, -1), (0, 1, 0)),
    Matrix((1, 0, 0), (0, -1, 0), (0, 0, -1)),
    Matrix((0, 1, 0), (1, 0, 0), (0, 0, -1)),
    Matrix((0, 1, 0), (0, 0, 1), (1, 0, 0)),
    Matrix((0, 1, 0), (0, 0, -1), (-1, 0, 0)),
    Matrix((0, 1, 0), (-1, 0, 0), (0, 0, 1)),
    Matrix((0, 0, 1), (1, 0, 0), (0, 1, 0)),
    Matrix((0, 0, 1), (0, 1, 0), (-1, 0, 0)),
    Matrix((0, 0, 1), (0, -1, 0), (1, 0, 0)),
    Matrix((0, 0, 1), (-1, 0, 0), (0, -1, 0)),
    Matrix((0, 0, -1), (1, 0, 0), (0, -1, 0)),
    Matrix((0, 0, -1), (0, 1, 0), (1, 0, 0)),
    Matrix((0, 0, -1), (0, -1, 0), (-1, 0, 0)),
    Matrix((0, 0, -1), (-1, 0, 0), (0, 1, 0)),
    Matrix((0, -1, 0), (1, 0, 0), (0, 0, 1)),
    Matrix((0, -1, 0), (0, 0, 1), (-1, 0, 0)),
    Matrix((0, -1, 0), (0, 0, -1), (1, 0, 0)),
    Matrix((0, -1, 0), (-1, 0, 0), (0, 0, -1)),
    Matrix((-1, 0, 0), (0, 1, 0), (0, 0, -1)),
    Matrix((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
    Matrix((-1, 0, 0), (0, 0, -1), (0, -1, 0)),
    Matrix((-1, 0, 0), (0, -1, 0), (0, 0, 1)),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PointDelta(i32, i32, i32);
impl PointDelta {
    fn size(&self) -> i32 {
        return self.0.abs() + self.1.abs() + self.2.abs();
    }

    fn matches(&self, rhs: &PointDelta, matrix: &Matrix) -> bool {
        if self.size() != rhs.size() {
            return false;
        }
        return *self * matrix == *rhs;
    }
}
impl Add<PointDelta> for PointDelta {
    type Output = PointDelta;

    fn add(self, rhs: PointDelta) -> Self::Output {
        return PointDelta(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}
impl Mul<&Matrix> for PointDelta {
    type Output = PointDelta;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let c0 = self.0 * rhs.0 .0 + self.1 * rhs.1 .0 + self.2 * rhs.2 .0;
        let c1 = self.0 * rhs.0 .1 + self.1 * rhs.1 .1 + self.2 * rhs.2 .1;
        let c2 = self.0 * rhs.0 .2 + self.1 * rhs.1 .2 + self.2 * rhs.2 .2;
        return Self(c0, c1, c2);
    }
}
impl PartialOrd for PointDelta {
    fn lt(&self, other: &Self) -> bool {
        return self.size().lt(&other.size());
    }

    fn le(&self, other: &Self) -> bool {
        return self.size().le(&other.size());
    }

    fn gt(&self, other: &Self) -> bool {
        return self.size().gt(&other.size());
    }

    fn ge(&self, other: &Self) -> bool {
        return self.size().ge(&other.size());
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.size().partial_cmp(&other.size());
    }
}
impl Ord for PointDelta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.size().cmp(&other.size());
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32, i32);
impl Point {
    fn apply(&self, matrix: &Matrix, offset: &PointDelta) -> Self {
        return &(self * matrix) + offset;
    }
}
impl Sub<&Point> for &Point {
    type Output = PointDelta;

    fn sub(self, rhs: &Point) -> Self::Output {
        return PointDelta(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}
impl Add<&PointDelta> for &Point {
    type Output = Point;

    fn add(self, rhs: &PointDelta) -> Self::Output {
        return Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}
impl Mul<&Matrix> for &Point {
    type Output = Point;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let PointDelta(c0, c1, c2) = PointDelta(self.0, self.1, self.2) * rhs;
        return Point(c0, c1, c2);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ScannerInput(u8, Vec<Point>);

// Scanners that have not yet been connected to the main frame of reference. The points in these are all in the original frame of reference (as in the input).
#[derive(Debug)]
struct ScannerIncomplete {
    pub num: u8,
    pub beacons: Vec<Point>,
    pub deltas: Vec<(PointDelta, Point, Point)>,
}
impl ScannerIncomplete {
    fn new(scanner: ScannerInput) -> Self {
        let beacons = scanner.1;
        // Make a preselection of deltas, largest first since larger deltas are more likely to involve points on the edge of the scanner area, which are in turn more likely to be found inside another scanner area.
        let mut deltas = beacons
            .iter()
            .flat_map(|l| {
                beacons
                    .iter()
                    .filter(|r| r != &l)
                    .map(|r| (l - r, *l, *r))
                    .collect::<Vec<(PointDelta, Point, Point)>>()
            })
            .collect::<Vec<(PointDelta, Point, Point)>>();
        deltas.sort_unstable_by_key(|d| d.0);
        deltas.resize(
            beacons.len() * 2,
            (PointDelta(0, 0, 0), Point(0, 0, 0), Point(0, 0, 0)),
        );
        return Self {
            num: scanner.0,
            deltas,
            beacons,
        };
    }
}

// Scanners that have been connected to the main frame of reference. The points in these are all transformed to the main frame of reference.
#[derive(Debug)]
struct Scanner {
    pub num: u8,
    pub offset: PointDelta,
    pub beacons: Vec<Point>,
    pub deltas: Vec<(PointDelta, Point, Point)>,
}
impl Scanner {
    fn new(scanner: &ScannerIncomplete, matrix: Matrix, offset: PointDelta) -> Self {
        let beacons: Vec<Point> = scanner
            .beacons
            .clone()
            .into_iter()
            .map(|p| p.apply(&matrix, &offset))
            .collect();
        let deltas = scanner
            .deltas
            .iter()
            .map(|(_, l, r)| {
                let l = l.apply(&matrix, &offset);
                let r = r.apply(&matrix, &offset);
                return (&l - &r, l, r);
            })
            .collect();
        return Self {
            num: scanner.num,
            offset,
            beacons,
            deltas,
        };
    }
}

fn parse_input(input: String) -> Vec<ScannerInput> {
    return input
        .trim()
        .split("\n\n")
        .map(|block| {
            let mut lines = block.trim().split("\n");

            let header = lines.next().unwrap();
            assert!(header.starts_with("--- scanner ") && header.ends_with(" ---"));
            let num = header[12..(header.len() - 4)].parse::<u8>().unwrap();

            let points = lines
                .into_iter()
                .map(|line| {
                    let mut coords = line
                        .trim()
                        .splitn(3, ",")
                        .map(str::parse)
                        .map(Result::unwrap);
                    return Point(
                        coords.next().unwrap(),
                        coords.next().unwrap(),
                        coords.next().unwrap(),
                    );
                })
                .collect();

            return ScannerInput(num, points);
        })
        .collect();
}

fn get_overlapping_deltas(
    existing: &Scanner,
    candidate: &ScannerIncomplete,
    matrix: &Matrix,
) -> Vec<(PointDelta, Point, Point)> {
    return candidate
        .deltas
        .iter()
        .flat_map(|(cd, cp, _)| {
            existing
                .deltas
                .iter()
                .find(|(ed, _, _)| cd.matches(ed, matrix))
                .map(|(_, ep, _)| (*cd, *cp, *ep))
        })
        .collect();
}

// Try to match a candidate incomplete scanner by comparing deltas. If any deltas match for a found matrix, go through the points and check if the overlap is large enough to fine a definitive match.
fn find_matrix_and_offset(
    existing: &Scanner,
    candidate: &ScannerIncomplete,
) -> Option<(Matrix, PointDelta)> {
    for matrix in &ROTATION_MATRICES {
        let overlapping_deltas = get_overlapping_deltas(&existing, &candidate, matrix);

        if overlapping_deltas.len() < MATCH_THRESHOLD - 1 {
            continue;
        }

        let pairs: HashSet<(Point, Point)> = overlapping_deltas
            .into_iter()
            .map(|(_, l, r)| (l, r))
            .collect();
        for (cp, ep) in &pairs {
            let cd: BTreeSet<PointDelta> = candidate
                .beacons
                .iter()
                .map(|p| (p - cp) * matrix)
                .collect();
            let ed: BTreeSet<PointDelta> = existing.beacons.iter().map(|p| p - ep).collect();
            let intersection = cd.intersection(&ed);
            if intersection.count() >= MATCH_THRESHOLD {
                let offset = ep - &(cp * matrix);
                return Some((*matrix, offset));
            }
        }
    }
    return None;
}

fn resolve(scanners: Vec<ScannerInput>) -> Vec<Scanner> {
    let mut remaining: Vec<ScannerIncomplete> =
        scanners.into_iter().map(ScannerIncomplete::new).collect();
    let mut solved: Vec<Scanner> = Vec::new();
    let mut failed: HashSet<(u8, u8)> = HashSet::new();

    // Take the first scanner to be fine as-is so we have something to start building against.
    solved.push(Scanner::new(
        &remaining.swap_remove(0),
        IDENTITY_MATRIX,
        PointDelta(0, 0, 0),
    ));

    'try_match: while !remaining.is_empty() {
        for (ci, candidate) in remaining.iter().enumerate() {
            for existing in &solved {
                if failed.contains(&(candidate.num, existing.num)) {
                    continue;
                }
                match find_matrix_and_offset(existing, candidate) {
                    Some((matrix, offset)) => {
                        let new_scanner = Scanner::new(candidate.clone(), matrix, offset);
                        solved.push(new_scanner);
                        remaining.swap_remove(ci);
                        continue 'try_match;
                    }
                    _ => {
                        failed.insert((candidate.num, existing.num));
                    }
                }
            }
        }

        panic!("Unable to match remaining scanners to existing ones.");
    }

    return solved;
}

fn get_beacons(scanners: &Vec<Scanner>) -> HashSet<Point> {
    let mut beacons: HashSet<Point> = HashSet::new();
    for scanner in scanners {
        for beacon in &scanner.beacons {
            beacons.insert(*beacon);
        }
    }
    return beacons;
}

fn part1(input: String) -> usize {
    let scanners = parse_input(input);
    let resolved = resolve(scanners);
    let beacons = get_beacons(&resolved);
    return beacons.len();
}

fn part2(input: String) -> i32 {
    let scanners = parse_input(input);
    let resolved = resolve(scanners);
    let start = Point(0, 0, 0);
    let mut max = 0;
    for s1 in &resolved {
        let p1 = &start + &s1.offset;
        for s2 in &resolved {
            let p2 = &start + &s2.offset;
            let distance = (&p1 - &p2).size();
            if distance > max {
                max = distance;
            }
        }
    }
    return max;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            ScannerInput(
                0,
                vec![
                    Point(404, -588, -901),
                    Point(528, -643, 409),
                    Point(-838, 591, 734),
                    Point(390, -675, -793),
                    Point(-537, -823, -458),
                    Point(-485, -357, 347),
                    Point(-345, -311, 381),
                    Point(-661, -816, -575),
                    Point(-876, 649, 763),
                    Point(-618, -824, -621),
                    Point(553, 345, -567),
                    Point(474, 580, 667),
                    Point(-447, -329, 318),
                    Point(-584, 868, -557),
                    Point(544, -627, -890),
                    Point(564, 392, -477),
                    Point(455, 729, 728),
                    Point(-892, 524, 684),
                    Point(-689, 845, -530),
                    Point(423, -701, 434),
                    Point(7, -33, -71),
                    Point(630, 319, -379),
                    Point(443, 580, 662),
                    Point(-789, 900, -551),
                    Point(459, -707, 401),
                ],
            ),
            ScannerInput(
                1,
                vec![
                    Point(686, 422, 578),
                    Point(605, 423, 415),
                    Point(515, 917, -361),
                    Point(-336, 658, 858),
                    Point(95, 138, 22),
                    Point(-476, 619, 847),
                    Point(-340, -569, -846),
                    Point(567, -361, 727),
                    Point(-460, 603, -452),
                    Point(669, -402, 600),
                    Point(729, 430, 532),
                    Point(-500, -761, 534),
                    Point(-322, 571, 750),
                    Point(-466, -666, -811),
                    Point(-429, -592, 574),
                    Point(-355, 545, -477),
                    Point(703, -491, -529),
                    Point(-328, -685, 520),
                    Point(413, 935, -424),
                    Point(-391, 539, -444),
                    Point(586, -435, 557),
                    Point(-364, -763, -893),
                    Point(807, -499, -711),
                    Point(755, -354, -619),
                    Point(553, 889, -390),
                ],
            ),
            ScannerInput(
                2,
                vec![
                    Point(649, 640, 665),
                    Point(682, -795, 504),
                    Point(-784, 533, -524),
                    Point(-644, 584, -595),
                    Point(-588, -843, 648),
                    Point(-30, 6, 44),
                    Point(-674, 560, 763),
                    Point(500, 723, -460),
                    Point(609, 671, -379),
                    Point(-555, -800, 653),
                    Point(-675, -892, -343),
                    Point(697, -426, -610),
                    Point(578, 704, 681),
                    Point(493, 664, -388),
                    Point(-671, -858, 530),
                    Point(-667, 343, 800),
                    Point(571, -461, -707),
                    Point(-138, -166, 112),
                    Point(-889, 563, -600),
                    Point(646, -828, 498),
                    Point(640, 759, 510),
                    Point(-630, 509, 768),
                    Point(-681, -892, -333),
                    Point(673, -379, -804),
                    Point(-742, -814, -386),
                    Point(577, -820, 562),
                ],
            ),
            ScannerInput(
                3,
                vec![
                    Point(-589, 542, 597),
                    Point(605, -692, 669),
                    Point(-500, 565, -823),
                    Point(-660, 373, 557),
                    Point(-458, -679, -417),
                    Point(-488, 449, 543),
                    Point(-626, 468, -788),
                    Point(338, -750, -386),
                    Point(528, -832, -391),
                    Point(562, -778, 733),
                    Point(-938, -730, 414),
                    Point(543, 643, -506),
                    Point(-524, 371, -870),
                    Point(407, 773, 750),
                    Point(-104, 29, 83),
                    Point(378, -903, -323),
                    Point(-778, -728, 485),
                    Point(426, 699, 580),
                    Point(-438, -605, -362),
                    Point(-469, -447, -387),
                    Point(509, 732, 623),
                    Point(647, 635, -688),
                    Point(-868, -804, 481),
                    Point(614, -800, 639),
                    Point(595, 780, -596),
                ],
            ),
            ScannerInput(
                4,
                vec![
                    Point(727, 592, 562),
                    Point(-293, -554, 779),
                    Point(441, 611, -461),
                    Point(-714, 465, -776),
                    Point(-743, 427, -804),
                    Point(-660, -479, -426),
                    Point(832, -632, 460),
                    Point(927, -485, -438),
                    Point(408, 393, -506),
                    Point(466, 436, -512),
                    Point(110, 16, 151),
                    Point(-258, -428, 682),
                    Point(-393, 719, 612),
                    Point(-211, -452, 876),
                    Point(808, -476, -593),
                    Point(-575, 615, 604),
                    Point(-485, 667, 467),
                    Point(-680, 325, -822),
                    Point(-627, -443, -432),
                    Point(872, -547, -609),
                    Point(833, 512, 582),
                    Point(807, 604, 487),
                    Point(839, -516, 451),
                    Point(891, -625, 532),
                    Point(-652, -548, -490),
                    Point(30, -46, -14),
                ],
            ),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_beacons() {
        let input = vec![
            Scanner {
                num: 0,
                offset: PointDelta(0, 0, 0),
                beacons: vec![
                    Point(404, -588, -901),
                    Point(528, -643, 409),
                    Point(-838, 591, 734),
                    Point(390, -675, -793),
                    Point(-537, -823, -458),
                ],
                deltas: vec![],
            },
            Scanner {
                num: 1,
                offset: PointDelta(0, 0, 0),
                beacons: vec![
                    Point(404, -588, -901),
                    Point(-485, -357, 347),
                    Point(-345, -311, 381),
                    Point(-537, -823, -458),
                    Point(-661, -816, -575),
                ],
                deltas: vec![],
            },
        ];
        let actual = get_beacons(&input);
        let expected: HashSet<Point> = vec![
            Point(404, -588, -901),
            Point(528, -643, 409),
            Point(-838, 591, 734),
            Point(390, -675, -793),
            Point(-537, -823, -458),
            Point(-485, -357, 347),
            Point(-345, -311, 381),
            Point(-661, -816, -575),
        ]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_resolve_get_beacons() {
        let input = vec![
            ScannerInput(
                0,
                vec![
                    Point(404, -588, -901),
                    Point(528, -643, 409),
                    Point(-838, 591, 734),
                    Point(390, -675, -793),
                    Point(-537, -823, -458),
                    Point(-485, -357, 347),
                    Point(-345, -311, 381),
                    Point(-661, -816, -575),
                    Point(-876, 649, 763),
                    Point(-618, -824, -621),
                    Point(553, 345, -567),
                    Point(474, 580, 667),
                    Point(-447, -329, 318),
                    Point(-584, 868, -557),
                    Point(544, -627, -890),
                    Point(564, 392, -477),
                    Point(455, 729, 728),
                    Point(-892, 524, 684),
                    Point(-689, 845, -530),
                    Point(423, -701, 434),
                    Point(7, -33, -71),
                    Point(630, 319, -379),
                    Point(443, 580, 662),
                    Point(-789, 900, -551),
                    Point(459, -707, 401),
                ],
            ),
            ScannerInput(
                1,
                vec![
                    Point(686, 422, 578),
                    Point(605, 423, 415),
                    Point(515, 917, -361),
                    Point(-336, 658, 858),
                    Point(95, 138, 22),
                    Point(-476, 619, 847),
                    Point(-340, -569, -846),
                    Point(567, -361, 727),
                    Point(-460, 603, -452),
                    Point(669, -402, 600),
                    Point(729, 430, 532),
                    Point(-500, -761, 534),
                    Point(-322, 571, 750),
                    Point(-466, -666, -811),
                    Point(-429, -592, 574),
                    Point(-355, 545, -477),
                    Point(703, -491, -529),
                    Point(-328, -685, 520),
                    Point(413, 935, -424),
                    Point(-391, 539, -444),
                    Point(586, -435, 557),
                    Point(-364, -763, -893),
                    Point(807, -499, -711),
                    Point(755, -354, -619),
                    Point(553, 889, -390),
                ],
            ),
            ScannerInput(
                2,
                vec![
                    Point(649, 640, 665),
                    Point(682, -795, 504),
                    Point(-784, 533, -524),
                    Point(-644, 584, -595),
                    Point(-588, -843, 648),
                    Point(-30, 6, 44),
                    Point(-674, 560, 763),
                    Point(500, 723, -460),
                    Point(609, 671, -379),
                    Point(-555, -800, 653),
                    Point(-675, -892, -343),
                    Point(697, -426, -610),
                    Point(578, 704, 681),
                    Point(493, 664, -388),
                    Point(-671, -858, 530),
                    Point(-667, 343, 800),
                    Point(571, -461, -707),
                    Point(-138, -166, 112),
                    Point(-889, 563, -600),
                    Point(646, -828, 498),
                    Point(640, 759, 510),
                    Point(-630, 509, 768),
                    Point(-681, -892, -333),
                    Point(673, -379, -804),
                    Point(-742, -814, -386),
                    Point(577, -820, 562),
                ],
            ),
            ScannerInput(
                3,
                vec![
                    Point(-589, 542, 597),
                    Point(605, -692, 669),
                    Point(-500, 565, -823),
                    Point(-660, 373, 557),
                    Point(-458, -679, -417),
                    Point(-488, 449, 543),
                    Point(-626, 468, -788),
                    Point(338, -750, -386),
                    Point(528, -832, -391),
                    Point(562, -778, 733),
                    Point(-938, -730, 414),
                    Point(543, 643, -506),
                    Point(-524, 371, -870),
                    Point(407, 773, 750),
                    Point(-104, 29, 83),
                    Point(378, -903, -323),
                    Point(-778, -728, 485),
                    Point(426, 699, 580),
                    Point(-438, -605, -362),
                    Point(-469, -447, -387),
                    Point(509, 732, 623),
                    Point(647, 635, -688),
                    Point(-868, -804, 481),
                    Point(614, -800, 639),
                    Point(595, 780, -596),
                ],
            ),
            ScannerInput(
                4,
                vec![
                    Point(727, 592, 562),
                    Point(-293, -554, 779),
                    Point(441, 611, -461),
                    Point(-714, 465, -776),
                    Point(-743, 427, -804),
                    Point(-660, -479, -426),
                    Point(832, -632, 460),
                    Point(927, -485, -438),
                    Point(408, 393, -506),
                    Point(466, 436, -512),
                    Point(110, 16, 151),
                    Point(-258, -428, 682),
                    Point(-393, 719, 612),
                    Point(-211, -452, 876),
                    Point(808, -476, -593),
                    Point(-575, 615, 604),
                    Point(-485, 667, 467),
                    Point(-680, 325, -822),
                    Point(-627, -443, -432),
                    Point(872, -547, -609),
                    Point(833, 512, 582),
                    Point(807, 604, 487),
                    Point(839, -516, 451),
                    Point(891, -625, 532),
                    Point(-652, -548, -490),
                    Point(30, -46, -14),
                ],
            ),
        ];
        let expected = vec![
            Point(-892, 524, 684),
            Point(-876, 649, 763),
            Point(-838, 591, 734),
            Point(-789, 900, -551),
            Point(-739, -1745, 668),
            Point(-706, -3180, -659),
            Point(-697, -3072, -689),
            Point(-689, 845, -530),
            Point(-687, -1600, 576),
            Point(-661, -816, -575),
            Point(-654, -3158, -753),
            Point(-635, -1737, 486),
            Point(-631, -672, 1502),
            Point(-624, -1620, 1868),
            Point(-620, -3212, 371),
            Point(-618, -824, -621),
            Point(-612, -1695, 1788),
            Point(-601, -1648, -643),
            Point(-584, 868, -557),
            Point(-537, -823, -458),
            Point(-532, -1715, 1894),
            Point(-518, -1681, -600),
            Point(-499, -1607, -770),
            Point(-485, -357, 347),
            Point(-470, -3283, 303),
            Point(-456, -621, 1527),
            Point(-447, -329, 318),
            Point(-430, -3130, 366),
            Point(-413, -627, 1469),
            Point(-345, -311, 381),
            Point(-36, -1284, 1171),
            Point(-27, -1108, -65),
            Point(7, -33, -71),
            Point(12, -2351, -103),
            Point(26, -1119, 1091),
            Point(346, -2985, 342),
            Point(366, -3059, 397),
            Point(377, -2827, 367),
            Point(390, -675, -793),
            Point(396, -1931, -563),
            Point(404, -588, -901),
            Point(408, -1815, 803),
            Point(423, -701, 434),
            Point(432, -2009, 850),
            Point(443, 580, 662),
            Point(455, 729, 728),
            Point(456, -540, 1869),
            Point(459, -707, 401),
            Point(465, -695, 1988),
            Point(474, 580, 667),
            Point(496, -1584, 1900),
            Point(497, -1838, -617),
            Point(527, -524, 1933),
            Point(528, -643, 409),
            Point(534, -1912, 768),
            Point(544, -627, -890),
            Point(553, 345, -567),
            Point(564, 392, -477),
            Point(568, -2007, -577),
            Point(605, -1665, 1952),
            Point(612, -1593, 1893),
            Point(630, 319, -379),
            Point(686, -3108, -505),
            Point(776, -3184, -501),
            Point(846, -3110, -434),
            Point(1135, -1161, 1235),
            Point(1243, -1093, 1063),
            Point(1660, -552, 429),
            Point(1693, -557, 386),
            Point(1735, -437, 1738),
            Point(1749, -1800, 1813),
            Point(1772, -405, 1572),
            Point(1776, -675, 371),
            Point(1779, -442, 1789),
            Point(1780, -1548, 337),
            Point(1786, -1538, 337),
            Point(1847, -1591, 415),
            Point(1889, -1729, 1762),
            Point(1994, -1805, 1792),
        ]
        .into_iter()
        .collect::<HashSet<Point>>();
        let actual = get_beacons(&resolve(input));
        assert_eq!(actual, expected);
    }

    #[test]
    fn matrix() {
        assert_eq!(
            &Matrix((0, 0, 1), (-1, 0, 0), (0, 1, 0)) * &Matrix((0, 0, 1), (-1, 0, 0), (0, 1, 0)),
            Matrix((0, 1, 0), (0, 0, -1), (-1, 0, 0)),
        );
        assert_eq!(
            &Point(1, 2, 3) * &Matrix((0, 0, 1), (-1, 0, 0), (0, 1, 0)),
            Point(-2, 3, 1)
        );
        assert_eq!(
            PointDelta(1, 2, 3) * &Matrix((0, 0, 1), (-1, 0, 0), (0, 1, 0)),
            PointDelta(-2, 3, 1)
        );
    }

    #[test]
    fn point_apply() {
        let matrix = Matrix((-1, 0, 0), (0, 1, 0), (0, 0, -1));
        let offset = PointDelta(68, -1246, -43);
        assert_eq!(
            Point(686, 422, 578).apply(&matrix, &offset),
            Point(-618, -824, -621)
        );
        assert_eq!(
            Point(686, 422, 578).apply(&matrix, &offset),
            Point(-618, -824, -621)
        );
        assert_eq!(
            Point(686, 422, 578).apply(&matrix, &offset),
            Point(-618, -824, -621)
        );
        assert_eq!(
            Point(605, 423, 415).apply(&matrix, &offset),
            Point(-537, -823, -458)
        );
        assert_eq!(
            Point(515, 917, -361).apply(&matrix, &offset),
            Point(-447, -329, 318)
        );
        assert_eq!(
            Point(-336, 658, 858).apply(&matrix, &offset),
            Point(404, -588, -901)
        );
        assert_eq!(
            Point(-476, 619, 847).apply(&matrix, &offset),
            Point(544, -627, -890)
        );
        assert_eq!(
            Point(-460, 603, -452).apply(&matrix, &offset),
            Point(528, -643, 409)
        );
        assert_eq!(
            Point(729, 430, 532).apply(&matrix, &offset),
            Point(-661, -816, -575)
        );
        assert_eq!(
            Point(-322, 571, 750).apply(&matrix, &offset),
            Point(390, -675, -793)
        );
        assert_eq!(
            Point(-355, 545, -477).apply(&matrix, &offset),
            Point(423, -701, 434)
        );
        assert_eq!(
            Point(413, 935, -424).apply(&matrix, &offset),
            Point(-345, -311, 381)
        );
        assert_eq!(
            Point(-391, 539, -444).apply(&matrix, &offset),
            Point(459, -707, 401)
        );
        assert_eq!(
            Point(553, 889, -390).apply(&matrix, &offset),
            Point(-485, -357, 347)
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 79);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 3621);
    }
}
