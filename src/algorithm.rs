use crate::cube::{Cube, Top};
use crate::moves::{parse_moves, Move};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Algorithm {
    X,
    XX,
    XXX,
    Sune,
    AntiSune,
    Sledge,
    AntiSledge,
    Stash,
}

use Algorithm::*;

pub const ALGORITHMS: [Algorithm; 8] = [X, XX, XXX, Sune, AntiSune, Sledge, AntiSledge, Stash];

impl Algorithm {
    pub fn is_extra(self) -> bool {
        match self {
            AntiSledge => true,
            _ => false
        }
    }

    pub fn moves_str(self) -> &'static str {
        match self {
            X => "F R U R' U' F'",
            XX => "F R U R' U' R U R' U' F'",
            XXX => "F R U R' U' R U R' U' R U R' U' F'",
            Sune => "R U R' U R U2 R'",
            AntiSune => "R U2 R' U' R U' R'",
            Sledge => "R U R' U' R' F R F'",
            AntiSledge => "F R' F' R U R U' R'",
            Stash => "F R U' R' U' R U R' F'"
        }
    }

    pub fn moves(self) -> Vec<Move> {
        parse_moves(self.moves_str())
    }

    pub fn top(self) -> Top {
        Cube::default().after_moves(&self.moves()).top()
    }
}
