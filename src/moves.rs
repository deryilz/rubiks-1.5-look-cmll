use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum MoveFace {
    U, R, F
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move {
    pub face: MoveFace,
    pub num: u8
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.face, match self.num {
            1 => "",
            2 => "2",
            3 => "'",
            n => unreachable!("Weird move num {}.", n)
        })
    }
}

fn parse_face(s: &str) -> MoveFace {
    match s {
        "U" => MoveFace::U,
        "R" => MoveFace::R,
        "F" => MoveFace::F,
        _ => panic!("Only implemented URF for now.")
    }
}

pub fn parse_move(s: &str) -> Move {
    if s.len() == 1 {
        Move { face: parse_face(s), num: 1 }
    } else if s.len() == 2 {
        let face = parse_face(&s[0..1]);
        match &s[1..2] {
            "2" => Move { face, num: 2 },
            "'" => Move { face, num: 3 },
            c => panic!("Unexpected modifier {}.", c)
        }
    } else {
        panic!("Unexpected length for {}.", s)
    }
}

pub fn parse_moves(s: &str) -> Vec<Move> {
    s.split(" ").map(parse_move).collect()
}

pub fn invert_move(m: Move) -> Move {
    Move { face: m.face, num: (4 - m.num) % 4 }
}

pub fn invert_moves(moves: &[Move]) -> Vec<Move> {
    moves.iter().cloned().map(invert_move).rev().collect()
}
