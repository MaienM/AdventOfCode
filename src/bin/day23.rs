use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt::Debug;

use aoc::runner::*;

// It's stupid that I cannot just return a goddamned iterator here.
fn range(start: usize, end: usize) -> Vec<usize> {
    if start < end {
        return (start..=end).into_iter().collect();
    } else {
        return (end..=start).into_iter().rev().collect();
    }
}

fn diff(start: usize, end: usize) -> usize {
    if start < end {
        return end - start;
    } else {
        return start - end;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Move {
    HallwayRoom(usize, (usize, usize)),
    RoomRoom((usize, usize), (usize, usize)),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MoveWithCost(u32, Move);

// The hallway spots that are valid to stop in.
type ValidStopPositions = [bool; 11];
const VALID_STOP_POSITIONS: ValidStopPositions = [
    true, true, false, true, false, true, false, true, false, true, true,
];
// The positions of the rooms.
const ROOM_POSITIONS: [usize; 4] = [2, 4, 6, 8];
// The solved state.
const ROOMS_SOLVED: [[Option<usize>; 2]; 4] =
    [[Some(0); 2], [Some(1); 2], [Some(2); 2], [Some(3); 2]];

#[derive(Clone, Eq, PartialEq, Hash)]
struct Board {
    pub hallway: [Option<usize>; 11],
    pub rooms: [[Option<usize>; 2]; 4],
}
impl Board {
    fn get_moves(&self) -> Vec<MoveWithCost> {
        let mut moves = Vec::new();

        // The rooms that can be moved into/out of + the seat that should be used.
        let mut rooms_move_into = [(false, 0); 4];
        let mut rooms_move_out = [(false, 0); 4];
        for room in 0..=3 {
            match self.rooms[room] {
                [Some(typ1), Some(typ2)] => {
                    if typ1 == room && typ2 == room {
                        // Room only contains the correct type and is done.
                    } else {
                        rooms_move_out[room] = (true, 0);
                    }
                }
                [None, Some(typ)] => {
                    if typ == room {
                        // Nothing to move out of the room, but there is still room.
                        rooms_move_into[room] = (true, 0);
                    } else {
                        rooms_move_out[room] = (true, 1);
                    }
                }
                _ => {
                    rooms_move_into[room] = (true, 1);
                }
            };
        }

        // Check if any of the amphipods can move directly from a room into their target room. If so this is the optimal move for this amphipod and we can ignore to/from hallway checks for both involved rooms.
        'source: for sourceroom in 0..=3 {
            let (sourceavail, sourceseat) = rooms_move_out[sourceroom];
            if !sourceavail {
                continue;
            }
            let typ = self.rooms[sourceroom][sourceseat].unwrap();

            let (targetavail, targetseat) = rooms_move_into[typ];
            if !targetavail {
                continue;
            }

            for pos in range(ROOM_POSITIONS[sourceroom], ROOM_POSITIONS[typ]) {
                if self.hallway[pos].is_some() {
                    continue 'source;
                }
            }

            let stepcost = 10u32.pow(typ as u32);
            let stepcount = (sourceseat + 1) + (targetseat + 1) + diff(sourceroom, typ) * 2;
            moves.push(MoveWithCost(
                stepcost * stepcount as u32,
                Move::RoomRoom((sourceroom, sourceseat), (typ, targetseat)),
            ));

            rooms_move_out[typ] = (false, 0);
            rooms_move_into[typ] = (false, 0);
        }

        // See if any of the amphipods can move out of their rooms into the hallway.
        for room in 0..=3 {
            let (avail, roomseat) = rooms_move_out[room];
            if !avail {
                continue;
            }
            let typ = self.rooms[room][roomseat].unwrap();

            let stepcost = 10u32.pow(typ as u32);
            let sourcepos = ROOM_POSITIONS[room];
            let roompos = ROOM_POSITIONS[typ];
            for targetrange in [range(sourcepos - 1, 0), range(sourcepos + 1, 10)] {
                'target: for targetpos in targetrange {
                    match self.hallway[targetpos] {
                        Some(_) => break,
                        None => {
                            if !VALID_STOP_POSITIONS[targetpos] {
                                continue;
                            }

                            // We need to check to see what is currenlty between us and our target. If whatever is between us and our target needs to pass us to get to its room this leads to a deadlock, so we can write off that move as an option.
                            for i in range(targetpos, roompos) {
                                match self.hallway[i] {
                                    Some(otyp) => {
                                        if range(i, ROOM_POSITIONS[otyp]).contains(&targetpos) {
                                            continue 'target;
                                        }
                                    }
                                    None => {}
                                }
                            }

                            let stepcount = (roomseat + 1) + diff(sourcepos, targetpos);
                            moves.push(MoveWithCost(
                                stepcost * stepcount as u32,
                                Move::HallwayRoom(targetpos, (room, roomseat)),
                            ));
                        }
                    }
                }
            }
        }

        // See if any of the amphipods can move into their rooms from the hallway.
        for room in 0..=3 {
            let (avail, roomseat) = rooms_move_into[room];
            if !avail {
                continue;
            }

            let stepcost = 10u32.pow(room as u32);
            let sourcepos = ROOM_POSITIONS[room];
            for range in [range(sourcepos - 1, 0), range(sourcepos + 1, 10)] {
                for targetpos in range {
                    match self.hallway[targetpos] {
                        Some(typ) => {
                            if typ == room {
                                let stepcount = (roomseat + 1) + diff(sourcepos, targetpos);
                                moves.push(MoveWithCost(
                                    stepcost * stepcount as u32,
                                    Move::HallwayRoom(targetpos, (room, roomseat)),
                                ));
                            }
                            break;
                        }
                        None => {}
                    }
                }
            }
        }

        return moves;
    }

    fn apply(&self, movkind: Move) -> Board {
        let mut result = self.clone();
        match movkind {
            Move::HallwayRoom(spot, (room, roomseat)) => {
                std::mem::swap(&mut result.rooms[room][roomseat], &mut result.hallway[spot]);
            }
            Move::RoomRoom((room1, room1seat), (room2, room2seat)) => {
                let tmp = std::mem::take(&mut result.rooms[room1][room1seat]);
                result.rooms[room1][room1seat] =
                    std::mem::replace(&mut result.rooms[room2][room2seat], tmp);
            }
        }
        return result;
    }

    fn is_solved(&self) -> bool {
        return self.rooms == ROOMS_SOLVED;
    }
}
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return writeln!(
            f,
            "#############\n#{}{}.{}.{}.{}.{}{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#\n  #########",
            self.hallway[0]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.hallway[1]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.hallway[3]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.hallway[5]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.hallway[7]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.hallway[9]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.hallway[10]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[0][0]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[1][0]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[2][0]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[3][0]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[0][1]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[1][1]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[2][1]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
            self.rooms[3][1]
                .map(|typ| (typ as u8 + b'A') as char)
                .unwrap_or('.'),
        );
    }
}

#[derive(Debug, Eq, PartialEq)]
struct BoardWithCost(Board, u32);
impl PartialOrd for BoardWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.1.partial_cmp(&self.1);
    }
}
impl Ord for BoardWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.1.cmp(&self.1);
    }
}

fn parse_input(input: String) -> Board {
    let mut rooms: [[Option<usize>; 2]; 4] = [[None; 2]; 4];
    let lines = input.trim().split("\n").collect::<Vec<&str>>();
    for roomseat in 0..=1 {
        let bytes = lines[2 + roomseat].trim().as_bytes();
        for room in 0..=3 {
            let byte = bytes[3 - roomseat * 2 + room * 2];
            rooms[room][roomseat] = Some((byte - b'A') as usize);
        }
    }

    return Board {
        hallway: [None; 11],
        rooms,
    };
}

pub fn part1(input: String) -> u32 {
    let board = parse_input(input);

    let mut heap: BinaryHeap<BoardWithCost> = BinaryHeap::new();
    let mut seen: HashSet<Board> = HashSet::new();
    heap.push(BoardWithCost(board, 0));

    while !heap.is_empty() {
        let BoardWithCost(board, cost) = heap.pop().unwrap();
        if seen.contains(&board) {
            continue;
        }

        if board.is_solved() {
            return cost;
        }

        for mov in board.get_moves() {
            let MoveWithCost(movcost, movkind) = mov.clone();
            let newboard = board.apply(movkind);
            heap.push(BoardWithCost(newboard, cost + movcost));
        }

        seen.insert(board);
    }

    panic!("Should never happen.");
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = Board {
            hallway: [None; 11],
            rooms: [
                [Some(1), Some(0)],
                [Some(2), Some(3)],
                [Some(1), Some(2)],
                [Some(3), Some(0)],
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_manual_sort() {
        let board = Board {
            hallway: [None; 11],
            rooms: [
                [Some(1), Some(0)],
                [Some(2), Some(3)],
                [Some(1), Some(2)],
                [Some(3), Some(0)],
            ],
        };
        let expected_mov = MoveWithCost(40, Move::HallwayRoom(3, (2, 0)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(400, Move::RoomRoom((1, 0), (2, 0)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(3000, Move::HallwayRoom(5, (1, 1)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(30, Move::HallwayRoom(3, (1, 1)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(40, Move::RoomRoom((0, 0), (1, 0)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(2000, Move::HallwayRoom(7, (3, 0)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(3, Move::HallwayRoom(9, (3, 1)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(3000, Move::HallwayRoom(7, (3, 1)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(4000, Move::HallwayRoom(5, (3, 0)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        let expected_mov = MoveWithCost(8, Move::HallwayRoom(9, (0, 0)));
        assert!(board.get_moves().contains(&expected_mov));

        let board = board.apply(expected_mov.1);
        assert!(board.is_solved());
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 12521);
    }
}
