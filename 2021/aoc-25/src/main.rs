use std::io::{stdin,BufRead};
use std::fmt;
use std::cmp;

#[derive(Copy, Clone,PartialEq)]
enum Block {
    EastFacing,
    SouthFacing,
    Empty
}

struct Map {
    rows: Vec<Vec<Block>>
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        for row in self.rows.iter() {
            for col in row.iter() {
                match col {
                    Block::EastFacing => f.write_str(">"),
                    Block::SouthFacing => f.write_str("v"),
                    Block::Empty => f.write_str(".")
                }?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl cmp::PartialEq for Map {
    fn eq(&self, other: &Map) -> bool { 
        let my_cols = self.rows.iter().flat_map(|r| r.iter());
        let other_cols = other.rows.iter().flat_map(|r| r.iter());
        my_cols.zip(other_cols).all(|(a, b)| a == b)
    }
}

impl Map {
    fn perform_move(&self) -> Map {
        let mut new_map = Map{rows: vec![vec![Block::Empty; self.rows[0].len()]; self.rows.len()]};
        for (y, row) in self.rows.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                match col {
                    Block::EastFacing => {
                        let new_x = (x + 1) % self.rows[0].len();
                        match self.rows[y][new_x] {
                            Block::Empty => new_map.rows[y][new_x] = Block::EastFacing,
                            _ => new_map.rows[y][x] = Block::EastFacing
                        }
                    }
                    _ => ()
                }
            }
        }
        for (y, row) in self.rows.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                match col {
                    Block::SouthFacing => {
                        let new_y = (y + 1) % self.rows.len();
                        match (self.rows[new_y][x], new_map.rows[new_y][x]) {
                            (Block::Empty, Block::Empty) => new_map.rows[new_y][x] = Block::SouthFacing,
                            (Block::EastFacing, Block::Empty) => new_map.rows[new_y][x] = Block::SouthFacing,
                            _ => new_map.rows[y][x] = Block::SouthFacing
                        }
                    }
                    _ => ()
                }
            }
        }
        new_map
    }
}

fn main() {
    let input: Vec<Vec<Block>> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars()
            .map(|c| match c {
                '>' => Block::EastFacing,
                'v' => Block::SouthFacing,
                _ => Block::Empty
            })
            .collect::<Vec<_>>())
        .collect();

    let mut map = Map{rows: input };

    println!("{}", map);
    let mut step_counter = 0;
    loop {
        step_counter += 1;
        let new_map = map.perform_move();
        println!("Step {}", step_counter);
        println!("{}", new_map);
        if new_map == map {
            break;
        }
        map = new_map;
    }
}
