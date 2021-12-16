use std::io::{stdin,BufRead};

use pathfinding::prelude::dijkstra;

type Cave=Vec<Vec<u32>>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
  fn successors(&self, cave: &Cave) -> Vec<(Pos, u32)> {
    let &Pos(x, y) = self;

    let w = cave[0].len();
    let h = cave.len();
    let mut result: Vec<(Pos, u32)> = Vec::new();

    if x < w - 1 {
        result.push((Pos(x + 1, y), cave[y][x+1]))
    }
    if y < h - 1 {
        result.push((Pos(x, y + 1), cave[y+1][x]))
    }
    if x > 0 {
        result.push((Pos(x - 1, y), cave[y][x-1]))
    }
    if y > 0 {
        result.push((Pos(x, y - 1), cave[y-1][x]))
    }
    result
  }
}


fn lowest_risk(cave: &Cave, x: usize, y: usize) -> u32
{
    let goal: Pos = Pos(cave[0].len() - 1, cave.len() - 1);
    let result = dijkstra(&Pos(x, y), |p| p.successors(cave), |p| *p == goal);
    result.expect("No path found").1
}

fn main() {
    let cave: Vec<Vec<u32>> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();
    
    let mut big_cave: Vec<Vec<u32>> = Vec::new();
    for ty in 0..5 {
        for y in 0..cave.len() {
            big_cave.push(Vec::new());
            for tx in 0..5 {
                for x in 0..cave[0].len() {
                    big_cave[ty*cave.len() + y].push((cave[y][x] - 1 + (ty as u32) + (tx as u32))%9 + 1);        
                }
            }
        }        
    }

    let lowest_risk_score = lowest_risk(&cave, 0, 0);
    println!("Lowest risk score (small cave): {}", lowest_risk_score);    

    let lowest_risk_score = lowest_risk(&big_cave, 0, 0);
    println!("Lowest risk score (big cave)  : {}", lowest_risk_score);    
}
