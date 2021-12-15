use std::collections::BinaryHeap;

use aoc::grid::{Grid as BaseGrid, Point};
use aoc::runner::*;

#[derive(Debug, Eq, PartialEq)]
struct Cell {
    pub point: Point,
    pub min_path_cost: i16,
}
impl Cell {
    pub fn new(point: Point, min_path_cost: i16) -> Self {
        return Self {
            point,
            min_path_cost,
        };
    }
}

// BinaryHeap is a max-heap, but we always want the smallest value, so we invert the ordering.
impl PartialOrd for Cell {
    fn lt(&self, other: &Self) -> bool {
        return other.min_path_cost.lt(&self.min_path_cost);
    }

    fn le(&self, other: &Self) -> bool {
        return other.min_path_cost.le(&self.min_path_cost);
    }

    fn gt(&self, other: &Self) -> bool {
        return other.min_path_cost.gt(&self.min_path_cost);
    }

    fn ge(&self, other: &Self) -> bool {
        return other.min_path_cost.ge(&self.min_path_cost);
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.min_path_cost.partial_cmp(&self.min_path_cost);
    }
}
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.min_path_cost.cmp(&self.min_path_cost);
    }
}

type InputGrid = BaseGrid<i8>;

fn parse_input(input: String) -> InputGrid {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            return line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>();
        })
        .collect();
}

fn calculate_min_path_cost(
    cost_grid: InputGrid,
    starting_point: Point,
    ending_point: Point,
) -> i16 {
    let mut min_cost_grid: BaseGrid<Option<i16>> =
        cost_grid.by_cell().map(|(p, _)| (p, None)).collect();
    let mut heap: BinaryHeap<Cell> = BinaryHeap::new();
    min_cost_grid.setp(starting_point, Some(0));
    heap.push(Cell::new(starting_point, 0));

    while !heap.is_empty() {
        let cell = heap.pop().unwrap();
        let next = cost_grid
            .neighbours(cell.point, false)
            .into_iter()
            .filter(|p| min_cost_grid.getp(*p).unwrap().is_none())
            .map(|p| (p, cost_grid.getp(p)))
            .min_by_key(|(_, c)| *c);
        if next.is_none() {
            continue;
        }

        let (next_point, next_cost) = next.unwrap();
        let cost = cell.min_path_cost + *next_cost.unwrap() as i16;
        if next_point == ending_point {
            return cost;
        }

        min_cost_grid.setp(next_point, Some(cost));
        heap.push(cell);
        heap.push(Cell::new(next_point, cost));
    }

    panic!("Should never happen.");
}

fn grow_grid(grid: InputGrid) -> InputGrid {
    let grid: InputGrid = grid
        .into_iter()
        .map(|row| {
            return row
                .iter()
                .map(|v| *v)
                .chain(row.iter().map(|v| v % 9 + 1))
                .chain(row.iter().map(|v| (v + 1) % 9 + 1))
                .chain(row.iter().map(|v| (v + 2) % 9 + 1))
                .chain(row.iter().map(|v| (v + 3) % 9 + 1))
                .collect::<Vec<i8>>();
        })
        .collect();
    let grid: InputGrid = grid
        .iter()
        .map(|row| row.iter().map(|v| *v).collect::<Vec<i8>>())
        .chain(
            grid.iter()
                .map(|row| row.iter().map(|v| v % 9 + 1).collect::<Vec<i8>>())
                .collect::<Vec<Vec<i8>>>(),
        )
        .chain(
            grid.iter()
                .map(|row| row.iter().map(|v| (v + 1) % 9 + 1).collect::<Vec<i8>>())
                .collect::<Vec<Vec<i8>>>(),
        )
        .chain(
            grid.iter()
                .map(|row| row.iter().map(|v| (v + 2) % 9 + 1).collect::<Vec<i8>>())
                .collect::<Vec<Vec<i8>>>(),
        )
        .chain(
            grid.iter()
                .map(|row| row.iter().map(|v| (v + 3) % 9 + 1).collect::<Vec<i8>>())
                .collect::<Vec<Vec<i8>>>(),
        )
        .collect();
    return grid;
}

fn part1(input: String) -> i16 {
    let grid = parse_input(input);
    let starting_point = Point::new(0, 0);
    let ending_point = Point::new(grid.width - 1, grid.height - 1);
    return calculate_min_path_cost(grid, starting_point, ending_point);
}

fn part2(input: String) -> i16 {
    let grid = parse_input(input);
    let grid = grow_grid(grid);
    let starting_point = Point::new(0, 0);
    let ending_point = Point::new(grid.width - 1, grid.height - 1);
    return calculate_min_path_cost(grid, starting_point, ending_point);
}

fn main() {
    run(part1, part2);
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT: &'static str = "
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected: BaseGrid<i8> = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
        .into();
        assert_eq!(actual, expected);
    }
    #[test]
    fn example_grow() {
        let actual = grow_grid(parse_input(EXAMPLE_INPUT.to_string()));
        let expected = parse_input(
            "
            11637517422274862853338597396444961841755517295286
            13813736722492484783351359589446246169155735727126
            21365113283247622439435873354154698446526571955763
            36949315694715142671582625378269373648937148475914
            74634171118574528222968563933317967414442817852555
            13191281372421239248353234135946434524615754563572
            13599124212461123532357223464346833457545794456865
            31254216394236532741534764385264587549637569865174
            12931385212314249632342535174345364628545647573965
            23119445813422155692453326671356443778246755488935
            22748628533385973964449618417555172952866628316397
            24924847833513595894462461691557357271266846838237
            32476224394358733541546984465265719557637682166874
            47151426715826253782693736489371484759148259586125
            85745282229685639333179674144428178525553928963666
            24212392483532341359464345246157545635726865674683
            24611235323572234643468334575457944568656815567976
            42365327415347643852645875496375698651748671976285
            23142496323425351743453646285456475739656758684176
            34221556924533266713564437782467554889357866599146
            33859739644496184175551729528666283163977739427418
            35135958944624616915573572712668468382377957949348
            43587335415469844652657195576376821668748793277985
            58262537826937364893714847591482595861259361697236
            96856393331796741444281785255539289636664139174777
            35323413594643452461575456357268656746837976785794
            35722346434683345754579445686568155679767926678187
            53476438526458754963756986517486719762859782187396
            34253517434536462854564757396567586841767869795287
            45332667135644377824675548893578665991468977611257
            44961841755517295286662831639777394274188841538529
            46246169155735727126684683823779579493488168151459
            54698446526571955763768216687487932779859814388196
            69373648937148475914825958612593616972361472718347
            17967414442817852555392896366641391747775241285888
            46434524615754563572686567468379767857948187896815
            46833457545794456865681556797679266781878137789298
            64587549637569865174867197628597821873961893298417
            45364628545647573965675868417678697952878971816398
            56443778246755488935786659914689776112579188722368
            55172952866628316397773942741888415385299952649631
            57357271266846838237795794934881681514599279262561
            65719557637682166874879327798598143881961925499217
            71484759148259586125936169723614727183472583829458
            28178525553928963666413917477752412858886352396999
            57545635726865674683797678579481878968159298917926
            57944568656815567976792667818781377892989248891319
            75698651748671976285978218739618932984172914319528
            56475739656758684176786979528789718163989182927419
            67554889357866599146897761125791887223681299833479
        "
            .to_string(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 40);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 315);
    }
}
