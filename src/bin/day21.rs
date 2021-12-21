#[derive(Debug, Clone, Copy)]
struct Player {
    pos: i32,
    score: i32,
}

struct DeterministicDie {
    next_roll: i32,
    min: i32,
    max: i32,
    rolls: u32,
}

impl DeterministicDie {
    fn roll(&mut self) -> i32 {
        let result = self.next_roll;

        if result >= self.max {
            self.next_roll = self.min;
        } else {
            self.next_roll += 1;
        }

        self.rolls += 1;

        result
    }

    fn rolls(&self) -> u32 {
        self.rolls
    }
}

fn main() {
    let mut die = DeterministicDie {
        next_roll: 1,
        min: 1,
        max: 100,
        rolls: 0,
    };

    // Player 1 starting position: 7
    // Player 2 starting position: 1
    let mut player1 = Player { pos: 7, score: 0 };
    let mut player2 = Player { pos: 1, score: 0 };

    loop {
        turn_1(&mut die, &mut player1);
        if player1.score >= 1000 {
            break;
        }
        turn_1(&mut die, &mut player2);
        if player2.score >= 1000 {
            break;
        }
    }

    println!(
        "Part 1: {}",
        die.rolls() as i32 * std::cmp::min(player1.score, player2.score)
    );

    let player1 = Player { pos: 7, score: 0 };
    let player2 = Player { pos: 1, score: 0 };
    let (player1_universes, player2_universes) = solve_2(player1, player2);

    println!(
        "Part 2: {:?}",
        std::cmp::max(player1_universes, player2_universes)
    );
}

fn turn_1(die: &mut DeterministicDie, player: &mut Player) {
    let d: i32 = (0..3).map(|_| die.roll()).sum();
    player.pos = (player.pos + d - 1) % 10 + 1;
    player.score += player.pos;
}

fn solve_2(player1: Player, player2: Player) -> (u64, u64) {
    if player1.score >= 21 {
        return (1, 0);
    }
    if player2.score >= 21 {
        return (0, 1);
    }

    const ROLLS: [(i32, u64); 7] = [(4, 3), (3, 1), (6, 7), (8, 3), (9, 1), (7, 6), (5, 6)];

    ROLLS.iter().fold((0, 0), |sum, (roll, count)| {
        let mut player1 = player1;

        player1.pos = (player1.pos + roll - 1) % 10 + 1;
        player1.score += player1.pos;

        let (u1, u2) = solve_2(player2, player1);

        (sum.0 + u2 * count, sum.1 + u1 * count)
    })
}
