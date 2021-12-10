use aoc::*;

type BaseBoard<T> = [[T; 5]; 5];
type Board = BaseBoard<i32>;

struct BoardSpaceState {
    num: i32,
    drawn: bool,
}
type BoardState = BaseBoard<BoardSpaceState>;

const WINNING_LINES: [[(usize, usize); 5]; 10] = [
    // Rows.
    [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
    [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)],
    [(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)],
    [(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)],
    [(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)],
    // Columns.
    [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
    [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)],
    [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)],
    [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)],
    [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)],
];

fn parse_input(input: String) -> (Vec<i32>, Vec<Board>) {
    let mut lines = input.trim().split("\n");

    let draws: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|p| p.trim().parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut current_block: Vec<[i32; 5]> = Vec::new();
    let mut expect_empty = true;
    for line in lines {
        if line.trim().is_empty() {
            assert!(current_block.is_empty());
            expect_empty = false;
            continue;
        }
        if expect_empty {
            panic!("Was expecting an empty line, got {}", line);
        }

        current_block.push(
            line.trim()
                .split(" ")
                .map(str::trim)
                .filter(|p| !p.is_empty())
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap(),
        );

        if current_block.len() == 5 {
            boards.push(current_block.clone().try_into().unwrap());
            current_block.clear();
            expect_empty = true;
        }
    }

    return (draws, boards);
}

fn init_board_state(board: Board) -> BoardState {
    return board.map(|row| {
        return row.map(|num| {
            return BoardSpaceState { num, drawn: false };
        });
    });
}

fn mark_number(state: &mut BoardState, draw: i32) {
    for space in state.iter_mut().flatten() {
        if space.num == draw {
            space.drawn = true;
        }
    }
}

fn is_winner(state: &BoardState) -> bool {
    for coords in WINNING_LINES {
        if coords.iter().all(|c| state[c.0][c.1].drawn) {
            return true;
        }
    }
    return false;
}

fn get_unmarked_sum(state: &BoardState) -> i32 {
    return state
        .iter()
        .flatten()
        .filter(|space| !space.drawn)
        .map(|space| space.num)
        .sum();
}

fn part1(input: String) -> i64 {
    let (draws, boards) = parse_input(input);
    let mut states: Vec<BoardState> = boards.into_iter().map(init_board_state).collect();
    for draw in draws {
        for state in states.iter_mut() {
            mark_number(state, draw);
            if is_winner(state) {
                let sum = get_unmarked_sum(state);
                return (sum * draw).into();
            }
        }
    }
    panic!("Bingo night ended, no one won.");
}

fn part2(input: String) -> i64 {
    let (draws, boards) = parse_input(input);
    let mut states: Vec<BoardState> = boards.into_iter().map(init_board_state).collect();
    for draw in draws {
        let mut winners: Vec<usize> = Vec::new();
        for (i, state) in states.iter_mut().enumerate() {
            mark_number(state, draw);
            if is_winner(state) {
                winners.push(i)
            }
        }

        if states.len() == 1 && winners.len() == 1 {
            let sum = get_unmarked_sum(&states[0]);
            return (sum * draw).into();
        }

        for idx in winners.iter().rev() {
            states.swap_remove(idx.to_owned());
        }
    }
    panic!("Bingo night ended, some boards never won.");
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    ";

    #[test]
    fn example_parse() {
        let (actual_draw, actual_boards) = parse_input(EXAMPLE_INPUT.to_string());
        let expected_draw = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let expected_boards = vec![
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ],
            [
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6],
            ],
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ],
        ];
        assert_eq!(actual_draw, expected_draw);
        assert_eq!(actual_boards, expected_boards);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 4512);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 1924);
    }
}
