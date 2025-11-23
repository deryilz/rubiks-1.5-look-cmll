mod cube;
mod moves;

use indexmap::IndexMap;

use cube::*;
use moves::*;

fn main() {
    // let cube = Cube::default().after_moves(&parse_moves("R U2 R' U' R U' R' R U R' U R U2 R'"));
    // dbg!(cube.top());
    // todo!();

    let algorithms = vec![
        parse_moves("R U R' U R U2 R'"),
        parse_moves("R U2 R' U' R U' R'"),
        parse_moves("F R U R' U' F'"),
        parse_moves("F R U R' U' R U R' U' F'"),
        parse_moves("F R U R' U' R U R' U' R U R' U' F'"),
        parse_moves("F R U' R' U' R U R' F'"),
        parse_moves("R U R' U' R' F R F'"),
    ];

    // the "extra" algo for special cases, not a core one
    let extras = vec![
        parse_moves("F R' F' R U R U' R'")
    ];

    let mut reached: IndexMap<Top, Vec<Move>> = IndexMap::from([
        (Cube::default().top_normalized(), vec![])
    ]);

    for _ in 0..2 {
        let mut new_ones = reached.clone();
        for (_, val) in &reached {
            for i in 0..4 {
                // extra algos shouldn't be the final one
                let extras = if val.len() > 0 {
                    &extras
                } else {
                    &vec![]
                };

                for algo in algorithms.iter().chain(extras.iter()) {
                    let moves = invert(algo);
                    let mut path = val.clone();
                    if i > 0 {
                        let m = parse_move(["U", "U2", "U'"][i - 1]);
                        path.push(m);
                    }
                    path.extend(&moves);
                    let cube = Cube::default().after_moves(&path);
                    new_ones.entry(cube.top_normalized()).or_insert(path);
                }
            }
        }
        reached = new_ones;
    }

    let mut entries: Vec<_> = reached.iter().collect();
    entries.sort_by_key(|(k, _)| k.map(|t| t.rotation));
    for (key, val) in entries {
        println!("{:?}", key.map(|c| (c.rotation, c.piece)));

        for m in val {
            print!("{:?} ", m);
        }
        println!();

        for m in invert(val) {
            print!("{:?} ", m);
        }
        println!();
        println!();
    }

    println!("{:?}", reached.len());

}
