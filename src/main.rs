mod cube;
mod moves;
mod algorithm;

use indexmap::IndexMap;
use indexmap::IndexSet;

use cube::*;
use moves::*;
use algorithm::*;

fn main() {
    let mut reached: IndexMap<Top, IndexSet<Vec<Move>>> = IndexMap::from([
        (Cube::default().top_normalized(), IndexSet::from([vec![]]))
    ]);

    for generation in 0..2 {
        let mut new_ones = reached.clone();
        for algo in ALGORITHMS {
            // extra algos should only be used in the first gen
            if generation != 0 && algo.is_extra() { continue };
            let moves = invert_moves(&algo.moves());

            for i in 0..4 {
                for (_, val) in &reached {
                    for path in val {
                        let mut path = path.clone();
                        if i > 0 && path.len() != 0 {
                            let m = parse_move(["U", "U2", "U'"][i - 1]);
                            path.push(m);
                        }
                        path.extend(&moves);
                        new_ones.entry(Cube::from(&path).top_normalized())
                            .or_insert(IndexSet::new())
                            .insert(path);
                    }
                }
            }
        }
        reached = new_ones;
    }

    let mut entries: Vec<(Top, Vec<Vec<Move>>)> = reached
        .into_iter()
        .map(|(k, set)| (k, set.into_iter().collect()))
        .collect();

    for entry in entries.iter_mut() {
        for moves in &mut entry.1 {
            let extra = Cube::from(moves).type_and_extra().1;
            moves.extend(extra);
        }
    }

    entries.sort_by_key(|(k, _)| k.map(|t| t.rotation));

    for (_, move_lists) in &entries {
        let first = Cube::from(&move_lists[0]);
        println!("{} type", first.type_and_extra().0);
        println!();

        print_top(&first.top_normalized());

        for moves in move_lists {
            let strings: Vec<_> = invert_moves(moves).iter().map(|m| format!("{:?}", m)).collect();
            let mut string = strings.join(" ");

            for algo in ALGORITHMS {
                string = string.replace(algo.moves_str(), &format!("{:?}", algo));
            }

            println!("Inverse: [{:?}]", moves);
            // println!("Verbose: {:?}", invert_moves(moves));
            println!("Solve: {}", string);
            println!();
        }

        for _ in 0..5 {
            println!();
        }
    }

    println!("{}", entries.len());
}

pub fn print_top(top: &Top) {
    use TopCornerPiece::*;

    let c = top.map(|corner| {
        let mut colors = match corner.piece {
            RF => ["🟨", "🟧", "🟦"],
            LF => ["🟨", "🟩", "🟧"],
            LB => ["🟨", "🟥", "🟩"],
            RB => ["🟨", "🟦", "🟥"],
            Other => unreachable!()
        };
        colors.rotate_left(corner.rotation as usize);
        colors
    });

    println!("⬛️{}⬛️{}⬛️", c[2][1], c[3][2]);
    println!("{}{}⬛️{}{}", c[2][2], c[2][0], c[3][0], c[3][1]);
    println!("⬛️⬛️⬛️⬛️⬛️");
    println!("{}{}⬛️{}{}", c[1][1], c[1][0], c[0][0], c[0][2]);
    println!("⬛️{}⬛️{}⬛️", c[1][2], c[0][1]);
}
