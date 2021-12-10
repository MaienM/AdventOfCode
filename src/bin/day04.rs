use aoc::*;

type BoardSpace = (i32, bool);
type Board = [[BoardSpace; 5]; 5];

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
    let mut current_block: Vec<[BoardSpace; 5]> = Vec::new();
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
                .map(|p| p.parse().unwrap())
                .map(|n| (n, false))
                .collect::<Vec<BoardSpace>>()
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

fn part1(input: String) -> i32 {
    let (draws, mut boards) = parse_input(input);
    for draw in draws {
        for board in boards.iter_mut() {
            for space in board.iter_mut().flatten() {
                if space.0 == draw {
                    space.1 = true;
                }
            }

            let mut winner = false;
            for coords in WINNING_LINES {
                if coords.iter().all(|c| board[c.0][c.1].1) {
                    winner = true;
                    break;
                }
            }

            if winner {
                let sum: i32 = board
                    .iter()
                    .flatten()
                    .filter(|space| !space.1)
                    .map(|space| space.0)
                    .sum();
                return sum * draw;
            }
        }
    }
    panic!("Bingo night ended, no one won.");
}

fn main() {
    run(part1, missing);
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
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 4512);
    }
}
