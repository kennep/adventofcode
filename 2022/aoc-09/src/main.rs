use std::io::{stdin, BufRead};
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    Left(i32),
    Down(i32),
    Up(i32),
    Right(i32)
}

impl Move {
    fn steps(&self) -> i32 {
        match self {
            Move::Left(n) | Move::Up(n) | Move::Down(n) | Move::Right(n) => *n
        }
    }

    fn perform_step(&self, coord: (i32, i32)) -> (i32, i32)
    {
        let (x, y) = coord;
        match self {
            Move::Left(_) => (x - 1, y),
            Move::Up(_) => (x, y - 1),
            Move::Right(_) => (x + 1, y),
            Move::Down(_) => (x, y + 1)
        }
    }
}

#[derive(Debug)]
struct MoveParseErr(String);

impl FromStr for Move {
    type Err = MoveParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split_whitespace();
        let direction_str = components.next().unwrap();
        let length = components.next().unwrap().parse::<i32>().unwrap();

        match direction_str {
            "R" => Ok(Move::Right(length)),
            "L" => Ok(Move::Left(length)),
            "U" => Ok(Move::Up(length)),
            "D" => Ok(Move::Down(length)),
            _ => Err(MoveParseErr(s.to_owned()))
        }
    }
}

fn update_tail_pos(current_head: (i32, i32), current_tail: (i32, i32)) -> (i32, i32)
{
    let (hx, hy) = current_head;
    let (tx, ty) = current_tail;

    let sx = match (hx, tx) {
        _ if tx - hx >= 2 => -1, // move left
        _ if hx - tx >= 2 => 1, // move right
        _ if tx - hx >= 1 && (ty-hy).abs() >= 2 => -1, // move left
        _ if hx - tx >= 1 && (ty-hy).abs() >= 2 => 1, // move right
        _ => 0
    };

    let sy = match (hy, ty) {
        _ if ty - hy >= 2 => -1, // move up
        _ if hy - ty >= 2 => 1, // move down
        _ if ty - hy >= 1 && (tx-hx).abs() >= 2 => -1, // move up
        _ if hy - ty >= 1 && (tx-hx).abs() >= 2 => 1, // move down
        _ => 0
    };

    (tx + sx, ty + sy)
}

fn simulate_moves(moves: &Vec<Move>, num_knots: usize) -> usize
{
    let mut visited_posititions: HashSet<(i32, i32)> = HashSet::new(); 
    let mut knots: Vec<(i32, i32)> = std::iter::repeat((0, 0)).take(num_knots).collect();

    visited_posititions.insert(knots[knots.len() - 1]);

    for mov in moves {
        let steps = mov.steps();
        for _ in 0..steps {
            knots[0] = mov.perform_step(knots[0]);
            for k in 1..num_knots {
                knots[k] = update_tail_pos(knots[k - 1], knots[k]);
            }
            visited_posititions.insert(knots[knots.len() - 1]);
        }        
    }

    visited_posititions.len()
}

fn main() {
    let moves: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Move>().unwrap())
        .collect();

    println!("Visited positions (2): {}", simulate_moves(&moves, 2));
    println!("Visited positions (10): {}", simulate_moves(&moves, 10));
}
