use crate::moves::{Move, MoveFace};

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Debug)]
pub enum TopCornerPiece {
    RF, RB, LF, LB, Other
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Debug)]
pub struct Corner {
    pub piece: TopCornerPiece,
    pub rotation: u8
}

impl Corner {
    pub fn twist(self) -> Self {
        Corner {
            piece: self.piece,
            rotation: ((self.rotation + 1) % 3)
        }
    }

    pub fn twist_counter(self) -> Self {
        Corner {
            piece: self.piece,
            rotation: ((self.rotation + 2) % 3)
        }
    }
}

// RF, LF, LB, RB
pub type Top = [Corner; 4];

// a cube that only stores corners
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Cube {
    urf: Corner,
    ulf: Corner,
    ulb: Corner,
    urb: Corner,
    drf: Corner,
    dlf: Corner,
    dlb: Corner,
    drb: Corner,
}

impl Cube {
    pub fn from(moves: &[Move]) -> Self {
        Cube::default().after_moves(moves)
    }

    pub fn from_top(top: &Top) -> Self {
        let [urf, ulf, ulb, urb] = *top;
        Cube { urf, ulf, ulb, urb, ..Cube::default() }
    }

    pub fn after_move(&self, m: Move) -> Self {
        (0..m.num).fold(self.clone(), |cube, _| {
            match m.face {
                MoveFace::U => Cube {
                    urf: cube.urb,
                    ulf: cube.urf,
                    ulb: cube.ulf,
                    urb: cube.ulb,
                    ..cube.clone()
                },
                MoveFace::R => Cube {
                    urf: cube.drf.twist_counter(),
                    urb: cube.urf.twist(),
                    drb: cube.urb.twist_counter(),
                    drf: cube.drb.twist(),
                    ..cube.clone()
                },
                MoveFace::F => Cube {
                    urf: cube.ulf.twist(),
                    drf: cube.urf.twist_counter(),
                    dlf: cube.drf.twist(),
                    ulf: cube.dlf.twist_counter(),
                    ..cube.clone()
                },
            }
        })
    }

    pub fn after_moves(&self, ms: &[Move]) -> Self {
        ms.iter().cloned().fold(self.clone(), |cube, m| cube.after_move(m))
    }

    pub fn top(&self) -> Top {
        [self.urf, self.ulf, self.ulb, self.urb]
    }

    pub fn top_matches(&self, other: &Self) -> bool {
        self.top_normalized() == other.top_normalized()
    }

    pub fn top_solved(&self) -> bool {
        self.top_matches(&Cube::default())
    }

    pub fn top_normalized(&self) -> Top {
        let mut top = self.top();
        let mut min = top.clone();

        let value = |t: Top| (t.map(|c| c.rotation), t.map(|c| c.piece));

        use TopCornerPiece::*;
        let next = |piece| match piece {
            RF => LF,
            LF => LB,
            LB => RB,
            RB => RF,
            _ => unreachable!("Top shouldn't have bottom corners.")
        };

        for i in 0..16 {
            for corner in top.iter_mut() {
                corner.piece = next(corner.piece);
            }
            if i != 0 && i % 4 == 0 {
                top.rotate_right(1);
            }
            if value(top) < value(min) {
                min = top;
            }
        }

        min
    }

    // returns the algorithm and optional extra move
    pub fn type_and_extra(&self) -> (&'static str, Option<Move>) {
        let setups = [
            ([0u8, 0, 0, 0], "O"),
            ([2, 1, 2, 1], "H"),
            ([2, 2, 1, 1], "Pi"),
            ([0, 0, 2, 1], "U"),
            ([0, 0, 1, 2], "T"),
            ([0, 2, 2, 2], "S"),
            ([1, 0, 1, 1], "As"),
            ([0, 2, 0, 1], "L"),
        ];

        let mut rots = self.top().map(|c| c.rotation);

        for i in 0..4 {
            for (setup_rots, name) in setups {
                if setup_rots == rots {
                    let extra_move = if i == 0 {
                        None
                    } else {
                        Some(Move { face: MoveFace::U, num: i })
                    };
                    return (name, extra_move);
                }
            }
            rots.rotate_right(1);
        }

        unreachable!()
    }
}

impl Default for Cube {
    fn default() -> Self {
        use TopCornerPiece::*;
        let basic = |piece| Corner { piece, rotation: 0 };
        Cube {
            urf: basic(RF),
            ulf: basic(LF),
            ulb: basic(LB),
            urb: basic(RB),
            drf: basic(Other),
            dlf: basic(Other),
            dlb: basic(Other),
            drb: basic(Other),
        }
    }
}
