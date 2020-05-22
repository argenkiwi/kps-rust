extern crate pancurses;

use pancurses::{endwin, initscr, Input, Window};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::error::Error;
use std::{cmp, fmt};

pub enum Move {
    Kick,
    Punch,
    Sweep,
    Crouch,
    Block,
    Jump,
}

impl Distribution<Move> for Standard {
    fn sample<T: Rng + ?Sized>(&self, rng: &mut T) -> Move {
        match rng.gen_range(0, 6) {
            0 => Move::Kick,
            1 => Move::Punch,
            2 => Move::Sweep,
            3 => Move::Crouch,
            4 => Move::Block,
            _ => Move::Jump,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match *self {
            Move::Kick => "Kick",
            Move::Punch => "Punch",
            Move::Sweep => "Sweep",
            Move::Crouch => "Crouch",
            Move::Block => "Block",
            Move::Jump => "Jump",
        };

        write!(f, "{}", label)
    }
}

enum Outcome {
    Win,
    Trade,
    Lose,
    Chip,
    Ouch,
    Dodge,
    Miss,
    Draw,
}

impl Outcome {
    pub fn value(&self) -> (i32, i32) {
        match *self {
            Outcome::Win => (0, -2),
            Outcome::Trade => (-1, -1),
            Outcome::Lose => (-2, 0),
            Outcome::Chip => (0, -1),
            Outcome::Ouch => (-1, 0),
            Outcome::Dodge => (1, 0),
            Outcome::Miss => (0, 1),
            Outcome::Draw => (0, 0),
        }
    }
}

struct Round {
    bars: (i32, i32),
}

impl Round {
    fn new() -> Round {
        Round { bars: (10, 10) }
    }

    fn is_finished(&self) -> bool {
        self.bars.0 <= 0 || self.bars.1 <= 0
    }
}

pub fn turn(left: &Move, right: &Move) -> (i32, i32) {
    match left {
        Move::Kick => match right {
            Move::Kick => Outcome::Trade,
            Move::Punch => Outcome::Win,
            Move::Sweep => Outcome::Lose,
            Move::Crouch => Outcome::Miss,
            Move::Block => Outcome::Draw,
            Move::Jump => Outcome::Chip,
        },
        Move::Punch => match right {
            Move::Kick => Outcome::Lose,
            Move::Punch => Outcome::Trade,
            Move::Sweep => Outcome::Win,
            Move::Crouch => Outcome::Chip,
            Move::Block => Outcome::Miss,
            Move::Jump => Outcome::Draw,
        },
        Move::Sweep => match right {
            Move::Kick => Outcome::Win,
            Move::Punch => Outcome::Lose,
            Move::Sweep => Outcome::Trade,
            Move::Crouch => Outcome::Draw,
            Move::Block => Outcome::Chip,
            Move::Jump => Outcome::Miss,
        },
        Move::Crouch => match right {
            Move::Kick => Outcome::Dodge,
            Move::Punch => Outcome::Ouch,
            Move::Sweep => Outcome::Draw,
            Move::Crouch => Outcome::Draw,
            Move::Block => Outcome::Draw,
            Move::Jump => Outcome::Draw,
        },
        Move::Block => match right {
            Move::Kick => Outcome::Draw,
            Move::Punch => Outcome::Dodge,
            Move::Sweep => Outcome::Ouch,
            Move::Crouch => Outcome::Draw,
            Move::Block => Outcome::Draw,
            Move::Jump => Outcome::Draw,
        },
        Move::Jump => match right {
            Move::Kick => Outcome::Ouch,
            Move::Punch => Outcome::Draw,
            Move::Sweep => Outcome::Dodge,
            Move::Crouch => Outcome::Draw,
            Move::Block => Outcome::Draw,
            Move::Jump => Outcome::Draw,
        },
    }
    .value()
}

fn render_bars(window: &Window, round: &Round) {
    window.printw("YOU ");

    let mut index = 0;
    while index < 10 {
        let ch = if index < 10 - round.bars.0 { '-' } else { '#' };
        window.printw(format!("{}", ch));
        index += 1;
    }

    window.printw(" VS ");

    let mut index = 0;
    while index < 10 {
        let ch = if index < round.bars.1 { '#' } else { '-' };
        window.printw(format!("{}", ch));
        index += 1;
    }

    window.printw(" CPU\n");
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut round = Round::new();

    let window = initscr();

    window.printw("Fight!\n");

    while !round.is_finished() {

        // Display bars.
        render_bars(&window, &round);

        window.printw("Press (k)ick, (p)unch, (s)weep, (c)rouch, (b)lock or (j)ump.\n");

        // Capture input
        let p1_move = match window.getch() {
            Some(Input::Character('k')) => Move::Kick,
            Some(Input::Character('p')) => Move::Punch,
            Some(Input::Character('s')) => Move::Sweep,
            Some(Input::Character('c')) => Move::Crouch,
            Some(Input::Character('b')) => Move::Block,
            Some(Input::Character('j')) => Move::Jump,
            _ => {
                window.clear();
                continue
            },
        };

        let p2_move = rand::random::<Move>();

        window.clear();

        window.printw(format!("{} - {}\n", &p1_move, &p2_move));

        // Play turn.
        let turn = turn(&p1_move, &p2_move);

        // Apply turn to bars.
        round.bars.0 = cmp::max(0, cmp::min(10, round.bars.0 + turn.0));
        round.bars.1 = cmp::max(0, cmp::min(10, round.bars.1 + turn.1));
    }

    render_bars(&window, &round);

    if round.bars.0 > round.bars.1 {
        window.printw("You win!");
    } else if round.bars.0 < round.bars.1 {
        window.printw("You lose!");
    } else {
        window.printw("Double KO!");
    }

    window.printw(" Press any key to exit.\n");
    window.getch();

    endwin();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kick() {
        assert_eq!((-1, -1), turn(&Move::Kick, &Move::Kick));
        assert_eq!((0, -2), turn(&Move::Kick, &Move::Punch));
        assert_eq!((-2, 0), turn(&Move::Kick, &Move::Sweep));
        assert_eq!((0, 1), turn(&Move::Kick, &Move::Crouch));
        assert_eq!((0, 0), turn(&Move::Kick, &Move::Block));
        assert_eq!((0, -1), turn(&Move::Kick, &Move::Jump));
    }

    #[test]
    fn punch() {
        assert_eq!((-2, 0), turn(&Move::Punch, &Move::Kick));
        assert_eq!((-1, -1), turn(&Move::Punch, &Move::Punch));
        assert_eq!((0, -2), turn(&Move::Punch, &Move::Sweep));
        assert_eq!((0, -1), turn(&Move::Punch, &Move::Crouch));
        assert_eq!((0, 1), turn(&Move::Punch, &Move::Block));
        assert_eq!((0, 0), turn(&Move::Punch, &Move::Jump));
    }

    #[test]
    fn sweep() {
        assert_eq!((0, -2), turn(&Move::Sweep, &Move::Kick));
        assert_eq!((-2, 0), turn(&Move::Sweep, &Move::Punch));
        assert_eq!((-1, -1), turn(&Move::Sweep, &Move::Sweep));
        assert_eq!((0, 0), turn(&Move::Sweep, &Move::Crouch));
        assert_eq!((0, -1), turn(&Move::Sweep, &Move::Block));
        assert_eq!((0, 1), turn(&Move::Sweep, &Move::Jump));
    }

    #[test]
    fn crouch() {
        assert_eq!((1, 0), turn(&Move::Crouch, &Move::Kick));
        assert_eq!((-1, 0), turn(&Move::Crouch, &Move::Punch));
        assert_eq!((0, 0), turn(&Move::Crouch, &Move::Sweep));
        assert_eq!((0, 0), turn(&Move::Crouch, &Move::Crouch));
        assert_eq!((0, 0), turn(&Move::Crouch, &Move::Block));
        assert_eq!((0, 0), turn(&Move::Crouch, &Move::Jump));
    }

    #[test]
    fn block() {
        assert_eq!((0, 0), turn(&Move::Block, &Move::Kick));
        assert_eq!((1, 0), turn(&Move::Block, &Move::Punch));
        assert_eq!((-1, 0), turn(&Move::Block, &Move::Sweep));
        assert_eq!((0, 0), turn(&Move::Block, &Move::Crouch));
        assert_eq!((0, 0), turn(&Move::Block, &Move::Block));
        assert_eq!((0, 0), turn(&Move::Block, &Move::Jump));
    }

    #[test]
    fn jump() {
        assert_eq!((-1, 0), turn(&Move::Jump, &Move::Kick));
        assert_eq!((0, 0), turn(&Move::Jump, &Move::Punch));
        assert_eq!((1, 0), turn(&Move::Jump, &Move::Sweep));
        assert_eq!((0, 0), turn(&Move::Jump, &Move::Crouch));
        assert_eq!((0, 0), turn(&Move::Jump, &Move::Block));
        assert_eq!((0, 0), turn(&Move::Jump, &Move::Jump));
    }
}
