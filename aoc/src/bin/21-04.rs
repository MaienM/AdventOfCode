puzzle_runner::register_chapter!(book = "2021", title = "Giant Squid");

type BaseBoard<T> = [[T; 5]; 5];
type Board = BaseBoard<u16>;

struct BoardSpaceState {
    num: u16,
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

fn parse_input(input: &str) -> (Vec<u16>, Vec<Board>) {
    parse!(input => {
        [draws split on ',' as u16]
        "\n\n"
        [boards split on "\n\n" with
            [split on '\n' try into (Board) with
                [split try into ([u16; 5]) try as u16]
            ]
        ]
    } => (draws, boards))
}

fn init_board_state(board: Board) -> BoardState {
    board.map(|row| row.map(|num| BoardSpaceState { num, drawn: false }))
}

fn mark_number(state: &mut BoardState, draw: u16) {
    for space in state.iter_mut().flatten() {
        if space.num == draw {
            space.drawn = true;
        }
    }
}

fn is_winner(state: &BoardState) -> bool {
    WINNING_LINES
        .iter()
        .any(|coords| coords.iter().all(|c| state[c.0][c.1].drawn))
}

fn get_unmarked_sum(state: &BoardState) -> u16 {
    state
        .iter()
        .flatten()
        .filter(|space| !space.drawn)
        .map(|space| space.num)
        .sum()
}

#[register_part]
fn part1(input: &str) -> u16 {
    let (draws, boards) = parse_input(input);
    let mut states: Vec<BoardState> = boards.into_iter().map(init_board_state).collect();
    for draw in draws {
        for state in &mut states {
            mark_number(state, draw);
            if is_winner(state) {
                let sum = get_unmarked_sum(state);
                return sum * draw;
            }
        }
    }
    panic!("Bingo night ended, no one won.");
}

#[register_part]
fn part2(input: &str) -> u16 {
    let (draws, boards) = parse_input(input);
    let mut states: Vec<BoardState> = boards.into_iter().map(init_board_state).collect();
    for draw in draws {
        let mut winners: Vec<usize> = Vec::new();
        for (i, state) in states.iter_mut().enumerate() {
            mark_number(state, draw);
            if is_winner(state) {
                winners.push(i);
            }
        }

        if states.len() == 1 && winners.len() == 1 {
            let sum = get_unmarked_sum(&states[0]);
            return sum * draw;
        }

        for idx in winners.iter().rev() {
            states.swap_remove(idx.to_owned());
        }
    }
    panic!("Bingo night ended, some boards never won.");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4512, part2 = 1924)]
    static EXAMPLE_INPUT: &str = "
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
        let (actual_draw, actual_boards) = parse_input(&EXAMPLE_INPUT);
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
}
