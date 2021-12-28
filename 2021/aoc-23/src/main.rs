use std::io::{stdin, BufRead};
use std::fmt;

#[derive(Copy,Clone)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self {
                Amphipod::Amber => 'A',
                Amphipod::Bronze => 'B',
                Amphipod::Copper => 'C',
                Amphipod::Desert => 'D'
            }
        )
    }
}

#[derive(Copy,Clone)]
enum Block {
    Wall,
    Space(Option<Amphipod>),
    HomeSpace(Option<Amphipod>, Amphipod),
    Blank
}

struct Board {
    rows: Vec<Vec<Block>>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            for col in row.iter() {
                col.fmt(f)?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::Wall => f.write_str("#")?,
            Block::Space(None) => f.write_str(".")?,
            Block::Space(Some(amphipod)) => amphipod.fmt(f)?,
            Block::HomeSpace(Some(amphipod), _) => amphipod.fmt(f)?,
            Block::HomeSpace(None, _) => f.write_str(".")?,
            Block::Blank => f.write_str(" ")?
        }
        Ok(())
    }
}


fn main() {
    let mut rows: Vec<Vec<Block>> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| match c {
                '#' => Block::Wall,
                '.' => Block::Space(None),
                'A' => Block::Space(Some(Amphipod::Amber)),
                'B' => Block::Space(Some(Amphipod::Bronze)),
                'C' => Block::Space(Some(Amphipod::Copper)),
                'D' => Block::Space(Some(Amphipod::Desert)),
                _ => Block::Blank
            })
            .collect::<Vec<_>>()
        )
        .collect();

    for row in rows.iter_mut() {
        let mut amphipods = vec![Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert];
        for col in row.iter_mut() {
            if let Block::Space(Some(amphipod)) = col {
                *col = Block::HomeSpace(Some(*amphipod), amphipods.remove(0));
            }
        }
    }

    let mut board = Board{rows};
    println!("{}", board);
}
