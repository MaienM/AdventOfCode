use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::fmt::Debug;

use aoc::range::range;
use aoc::runner::*;

fn diff(start: usize, end: usize) -> usize {
    return if start < end {
        end - start
    } else {
        start - end
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Move {
    HallwayRoom(usize, (usize, usize)),
    RoomRoom((usize, usize), (usize, usize)),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MoveWithCost(u32, Move);

// The hallway spots that are valid to stop in.
const VALID_STOP_POSITIONS: [bool; 11] = [
    true, true, false, true, false, true, false, true, false, true, true,
];
// The positions of the rooms.
const ROOM_POSITIONS: [usize; 4] = [2, 4, 6, 8];

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Board<const SEATS: usize> {
    pub hallway: [Option<usize>; 11],
    pub rooms: [[Option<usize>; SEATS]; 4],
}
impl<const SEATS: usize> Board<SEATS> {
    fn get_moves(&self) -> Vec<MoveWithCost> {
        let mut moves = Vec::new();

        // The rooms that can be moved into/out of + the seat that should be used.
        let mut rooms_move_into = [(false, 0); 4];
        let mut rooms_move_out = [(false, 0); 4];
        'room: for room in 0..=3 {
            for seat in 0..SEATS {
                match self.rooms[room][seat] {
                    Some(typ) => {
                        if typ != room {
                            // Room contains something of the wrong type, so move out whatever the first filled seat is.
                            for seat in 0..SEATS {
                                if self.rooms[room][seat].is_some() {
                                    rooms_move_out[room] = (true, seat);
                                    rooms_move_into[room] = (false, 0);
                                    continue 'room;
                                }
                            }
                        }
                    }
                    None => {
                        // Empty spot, enable moving into. If there are futher empty seats or things that need to move out this will be overwritten again.
                        rooms_move_into[room] = (true, seat);
                    }
                }
            }
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
            'target: for targetrange in [range(sourcepos - 1, 0), range(sourcepos + 1, 10)] {
                for targetpos in targetrange {
                    match self.hallway[targetpos] {
                        Some(_) => break,
                        None => {
                            if !VALID_STOP_POSITIONS[targetpos] {
                                continue;
                            }

                            // We need to check to see what is currently between us and our target. If whatever is between us and our target needs to pass us to get to its room this leads to a deadlock, so we can write off that move as an option.
                            for i in range(targetpos, roompos) {
                                match self.hallway[i] {
                                    Some(otyp) => {
                                        if range(i, ROOM_POSITIONS[otyp]).contains(targetpos) {
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

    fn apply(&self, movkind: Move) -> Board<SEATS> {
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
        for room in 0..=3 {
            for seat in 0..SEATS {
                if self.rooms[room][seat] != Some(room) {
                    return false;
                }
            }
        }
        return true;
    }
}
impl<const SEATS: usize> Debug for Board<SEATS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fc = |cell: Option<usize>| -> char {
            return cell.map(|typ| (typ as u8 + b'A') as char).unwrap_or('.');
        };

        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            fc(self.hallway[0]),
            fc(self.hallway[1]),
            fc(self.hallway[3]),
            fc(self.hallway[5]),
            fc(self.hallway[7]),
            fc(self.hallway[9]),
            fc(self.hallway[10]),
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            fc(self.rooms[0][0]),
            fc(self.rooms[1][0]),
            fc(self.rooms[2][0]),
            fc(self.rooms[3][0]),
        )?;
        for seat in 1..SEATS {
            writeln!(
                f,
                "  #{}#{}#{}#{}#  ",
                fc(self.rooms[0][seat]),
                fc(self.rooms[1][seat]),
                fc(self.rooms[2][seat]),
                fc(self.rooms[3][seat]),
            )?;
        }
        return Ok(());
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PendingMove<const SEATS: usize>(u32, Box<Board<SEATS>>, Move);
impl<const SEATS: usize> PartialOrd for PendingMove<SEATS> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.0.partial_cmp(&self.0);
    }
}
impl<const SEATS: usize> Ord for PendingMove<SEATS> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.0.cmp(&self.0);
    }
}

fn get_best_moveset_cost<const SEATS: usize>(board: Board<SEATS>) -> u32 {
    let mut heap: BinaryHeap<PendingMove<SEATS>> = BinaryHeap::new();
    let mut seen: BTreeSet<Board<SEATS>> = BTreeSet::new();

    let dummymove = Move::RoomRoom((0, 0), (1, 0));
    heap.push(PendingMove(
        0,
        Box::new(board.apply(dummymove.clone())),
        dummymove,
    ));

    while !heap.is_empty() {
        let PendingMove(cost, board, mov) = heap.pop().unwrap();

        let board = board.apply(mov);
        if seen.contains(&board) {
            continue;
        }

        if board.is_solved() {
            return cost;
        }

        for MoveWithCost(movcost, mov) in board.get_moves() {
            heap.push(PendingMove(cost + movcost, Box::new(board.clone()), mov));
        }

        seen.insert(board);
    }

    panic!("Should never happen.");
}

fn parse_input(input: String) -> Board<2> {
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
    return get_best_moveset_cost(board);
}

pub fn part2(input: String) -> u32 {
    let board = parse_input(input);
    let board = Board::<4> {
        hallway: board.hallway,
        rooms: [
            [board.rooms[0][0], Some(3), Some(3), board.rooms[0][1]],
            [board.rooms[1][0], Some(2), Some(1), board.rooms[1][1]],
            [board.rooms[2][0], Some(1), Some(0), board.rooms[2][1]],
            [board.rooms[3][0], Some(0), Some(2), board.rooms[3][1]],
        ],
    };
    return get_best_moveset_cost(board);
}

fn main() {
    run(part1, part2);
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

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 44169);
    }
}
