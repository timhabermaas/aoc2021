use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum AmphipodType {
    A,
    B,
    C,
    D,
}

impl AmphipodType {
    fn cost(&self) -> usize {
        match self {
            AmphipodType::A => 1,
            AmphipodType::B => 10,
            AmphipodType::C => 100,
            AmphipodType::D => 1000,
        }
    }

    // #############
    // #01234567890#
    // ###A#B#C#D###
    //   #########
    fn index_above_room(&self) -> usize {
        match self {
            AmphipodType::A => 2,
            AmphipodType::B => 4,
            AmphipodType::C => 6,
            AmphipodType::D => 8,
        }
    }
}

fn part_1() -> usize {
    use AmphipodType::*;

    let initial_board_ex: Board = Board {
        room_a: vec![A, B],
        room_b: vec![D, C],
        room_c: vec![C, B],
        room_d: vec![A, D],
        floor: [
            None, None, None, None, None, None, None, None, None, None, None,
        ],
        room_height: 2,
    };

    // #############
    // #...........#
    // ###A#C#B#D###
    //   #B#A#D#C#
    //   #########
    let initial_board: Board = Board {
        room_a: vec![B, A],
        room_b: vec![A, C],
        room_c: vec![D, B],
        room_d: vec![C, D],
        floor: [
            None, None, None, None, None, None, None, None, None, None, None,
        ],
        room_height: 2,
    };

    find_shortest_path(initial_board).unwrap()
}

fn part_2() -> usize {
    use AmphipodType::*;

    let initial_board_ex: Board = Board {
        room_a: vec![A, D, D, B],
        room_b: vec![D, B, C, C],
        room_c: vec![C, A, B, B],
        room_d: vec![A, C, A, D],
        floor: [
            None, None, None, None, None, None, None, None, None, None, None,
        ],
        room_height: 4,
    };

    // #############
    // #...........#
    // ###A#C#B#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #B#A#D#C#
    //   #########
    let initial_board: Board = Board {
        room_a: vec![B, D, D, A],
        room_b: vec![A, B, C, C],
        room_c: vec![D, A, B, B],
        room_d: vec![C, C, A, D],
        floor: [
            None, None, None, None, None, None, None, None, None, None, None,
        ],
        room_height: 4,
    };

    find_shortest_path(initial_board).unwrap()
}

#[derive(Debug, Clone, Eq)]
struct Node {
    cost: usize,
    state: Board,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.state == other.state
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(start: Board) -> Option<usize> {
    let mut costs: HashMap<Board, usize> = HashMap::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();

    queue.push(Node {
        state: start.clone(),
        cost: 0,
    });
    costs.insert(start, 0);

    while let Some(Node { state, cost }) = queue.pop() {
        if state.is_solved() {
            return Some(cost);
        }

        if cost > *costs.get(&state).unwrap() {
            continue;
        }

        for (new_board, new_cost) in state
            .move_to_hall()
            .iter()
            .chain(state.move_in_room().iter())
        {
            let new_node = Node {
                state: new_board.clone(),
                cost: new_cost + cost,
            };

            if new_node.cost < *costs.get(&new_node.state).unwrap_or(&usize::MAX) {
                costs.insert(new_board.clone(), new_node.cost);
                queue.push(new_node);
            }
        }
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    room_a: Vec<AmphipodType>,
    room_b: Vec<AmphipodType>,
    room_c: Vec<AmphipodType>,
    room_d: Vec<AmphipodType>,
    floor: [Option<AmphipodType>; 11],
    room_height: usize,
}

fn clamp_range(min: usize, max: usize) -> std::ops::Range<usize> {
    if min <= max {
        min..max
    } else {
        min..min
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.checked_sub(b).unwrap_or_else(|| b - a)
}

impl Board {
    fn is_floor_empty(&self) -> bool {
        self.floor.iter().all(|x| x.is_none())
    }

    fn is_room_correct(&self, kind: AmphipodType) -> bool {
        self.room(kind).iter().all(|a| *a == kind)
    }

    fn is_solved(&self) -> bool {
        self.is_floor_empty()
            && self.is_room_correct(AmphipodType::A)
            && self.is_room_correct(AmphipodType::B)
            && self.is_room_correct(AmphipodType::C)
            && self.is_room_correct(AmphipodType::D)
    }

    fn room_mut(&mut self, kind: AmphipodType) -> &mut Vec<AmphipodType> {
        match kind {
            AmphipodType::A => &mut self.room_a,
            AmphipodType::B => &mut self.room_b,
            AmphipodType::C => &mut self.room_c,
            AmphipodType::D => &mut self.room_d,
        }
    }

    fn room(&self, kind: AmphipodType) -> &Vec<AmphipodType> {
        match kind {
            AmphipodType::A => &self.room_a,
            AmphipodType::B => &self.room_b,
            AmphipodType::C => &self.room_c,
            AmphipodType::D => &self.room_d,
        }
    }

    fn move_to_hall(&self) -> Vec<(Board, usize)> {
        let mut result = vec![];

        // Go through each room and move top amphipod into hallway. Unless entire room is full of
        // that amphipod's type.
        for target in [
            AmphipodType::A,
            AmphipodType::B,
            AmphipodType::C,
            AmphipodType::D,
        ] {
            // We don't want to remove amphipods from a room if they are already in their correct
            // room.
            if self.is_room_correct(target) {
                continue;
            }

            let index_above_room = target.index_above_room();
            // Find all indices left and right of the spot above the room until the spot is no
            // longer empty.
            let floor_targets = (0..index_above_room)
                .rev()
                .take_while(|i| self.floor[*i] == None)
                .chain((index_above_room + 1..=10).take_while(|i| self.floor[*i] == None))
                .filter(|i| *i != 2 && *i != 4 && *i != 6 && *i != 8)
                .collect::<Vec<_>>();

            for floor_index in floor_targets {
                let mut new_state = self.clone();

                let cost_room = new_state.room_height - self.room(target).len() + 1;
                let cost_floor = abs_diff(index_above_room, floor_index);

                let room = new_state.room_mut(target);

                // SAFETY: Can't be empty, since an empty room is considered "correct" and would
                // lead to a skip earlier.
                let amphipod = room.pop().unwrap();

                new_state.floor[floor_index] = Some(amphipod);

                result.push((new_state, (cost_room + cost_floor) * amphipod.cost()));
            }
        }

        result
    }

    fn move_in_room(&self) -> Vec<(Board, usize)> {
        let mut result = vec![];

        for (i, amp) in self.floor.iter().enumerate() {
            if let Some(kind) = amp {
                let room = self.room(*kind);

                if room.iter().all(|x| x == kind) {
                    let index_above_room = kind.index_above_room();
                    if self.floor[clamp_range(index_above_room, i)]
                        .iter()
                        .all(|a| *a == None)
                        && self.floor[clamp_range(i + 1, index_above_room)]
                            .iter()
                            .all(|a| *a == None)
                    {
                        let mut new_state = self.clone();

                        new_state.floor[i] = None;

                        new_state.room_mut(*kind).push(*kind);

                        let distance_floor = abs_diff(i, index_above_room);
                        let distance_room = self.room_height - room.len();

                        result.push((new_state, (distance_floor + distance_room) * kind.cost()));
                    }
                }
            }
        }

        result
    }
}

fn main() {
    println!("Part 1: {:?}", part_1());

    println!("Part 2: {:?}", part_2());
}

#[cfg(test)]
mod tests {
    use super::AmphipodType::*;
    use super::*;

    #[test]
    fn test_move_in_room_empty_room() {
        let board: Board = Board {
            room_a: vec![],
            room_b: vec![D, B, C, C],
            room_c: vec![C, A, B, B],
            room_d: vec![A, C, A, D],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            room_height: 4,
        };

        let board_expected: Board = Board {
            room_a: vec![A],
            room_b: vec![D, B, C, C],
            room_c: vec![C, A, B, B],
            room_d: vec![A, C, A, D],
            floor: [
                None, None, None, None, None, None, None, None, None, None, None,
            ],
            room_height: 4,
        };

        assert_eq!(board.move_in_room(), vec![(board_expected, 6)]);
    }

    #[test]
    fn test_move_in_room_full_of_as() {
        let board: Board = Board {
            room_a: vec![A, A],
            room_b: vec![D, B, C, C],
            room_c: vec![C, A, B, B],
            room_d: vec![A, C, A, D],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            room_height: 4,
        };

        let board_expected: Board = Board {
            room_a: vec![A, A, A],
            room_b: vec![D, B, C, C],
            room_c: vec![C, A, B, B],
            room_d: vec![A, C, A, D],
            floor: [
                None, None, None, None, None, None, None, None, None, None, None,
            ],
            room_height: 4,
        };

        assert_eq!(board.move_in_room(), vec![(board_expected, 4)]);
    }

    #[test]
    fn test_move_in_room_blocked() {
        let board: Board = Board {
            room_a: vec![],
            room_b: vec![D, B, C, C],
            room_c: vec![C],
            room_d: vec![A, C, A, D],
            floor: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(C),
                Some(A),
            ],
            room_height: 4,
        };

        let board_expected: Board = Board {
            room_a: vec![],
            room_b: vec![D, B, C, C],
            room_c: vec![C, C],
            room_d: vec![A, C, A, D],
            floor: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(A),
            ],
            room_height: 4,
        };

        assert_eq!(board.move_in_room(), vec![(board_expected, 600)]);
    }

    #[test]
    fn test_move_in_room_multiple() {
        let board: Board = Board {
            room_a: vec![],
            room_b: vec![D, B, C, C],
            room_c: vec![],
            room_d: vec![A, C, A, D],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(C),
            ],
            room_height: 4,
        };

        let board_expected_1: Board = Board {
            room_a: vec![A],
            room_b: vec![D, B, C, C],
            room_c: vec![],
            room_d: vec![A, C, A, D],
            floor: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(C),
            ],
            room_height: 4,
        };

        let board_expected_2: Board = Board {
            room_a: vec![],
            room_b: vec![D, B, C, C],
            room_c: vec![C],
            room_d: vec![A, C, A, D],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            room_height: 4,
        };

        assert_eq!(
            board.move_in_room(),
            vec![(board_expected_1, 6), (board_expected_2, 800)]
        );
    }

    #[test]
    fn test_move_to_hall() {
        // #############
        // #A....B....D#
        // ###.#.#C#.###
        //   #.#C#A#.#
        //   #A#B#C#.#
        //   #A#D#A#.#
        //   #########
        let board: Board = Board {
            room_a: vec![A, A],
            room_b: vec![D, B, C],
            room_c: vec![A, C, A, C],
            room_d: vec![],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                Some(B),
                None,
                None,
                None,
                None,
                Some(D),
            ],
            room_height: 4,
        };

        let board_expected_1: Board = Board {
            room_a: vec![A, A],
            room_b: vec![D, B],
            room_c: vec![A, C, A, C],
            room_d: vec![],
            floor: [
                Some(A),
                None,
                None,
                Some(C),
                None,
                Some(B),
                None,
                None,
                None,
                None,
                Some(D),
            ],
            room_height: 4,
        };

        let board_expected_2: Board = Board {
            room_a: vec![A, A],
            room_b: vec![D, B],
            room_c: vec![A, C, A, C],
            room_d: vec![],
            floor: [
                Some(A),
                Some(C),
                None,
                None,
                None,
                Some(B),
                None,
                None,
                None,
                None,
                Some(D),
            ],
            room_height: 4,
        };

        let board_expected_3: Board = Board {
            room_a: vec![A, A],
            room_b: vec![D, B, C],
            room_c: vec![A, C, A],
            room_d: vec![],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                Some(B),
                None,
                Some(C),
                None,
                None,
                Some(D),
            ],
            room_height: 4,
        };
        let board_expected_4: Board = Board {
            room_a: vec![A, A],
            room_b: vec![D, B, C],
            room_c: vec![A, C, A],
            room_d: vec![],
            floor: [
                Some(A),
                None,
                None,
                None,
                None,
                Some(B),
                None,
                None,
                None,
                Some(C),
                Some(D),
            ],
            room_height: 4,
        };

        let result = board.move_to_hall();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (board_expected_1, 300));
        assert_eq!(result[1], (board_expected_2, 500));
        assert_eq!(result[2], (board_expected_3, 200));
        assert_eq!(result[3], (board_expected_4, 400));
    }
}
