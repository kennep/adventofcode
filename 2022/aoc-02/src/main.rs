use std::{io::{stdin, BufRead}, str::FromStr};

#[derive(Debug,Clone,Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Move {
    type Err = MoveFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(MoveFormatError(s.to_owned()))
        }
    }
}

#[derive(Debug)]
enum DesiredResult {
    Win,
    Loose,
    Draw
}

impl FromStr for DesiredResult {
    type Err = MoveFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(DesiredResult::Loose),
            "Y" => Ok(DesiredResult::Draw),
            "Z" => Ok(DesiredResult::Win),
            _ => Err(MoveFormatError(s.to_owned()))
        }
    }
}


#[derive(Debug)]
struct MoveFormatError(String);

#[derive(Debug)]
struct RoundA(Move, Move);

impl RoundA
{
    fn points(&self) -> i32
    {
        let win_points = match self {
            RoundA(Move::Rock, Move::Rock) => 3,
            RoundA(Move::Rock, Move::Paper) => 6,
            RoundA(Move::Rock, Move::Scissors) => 0,
            RoundA(Move::Scissors, Move::Rock) => 6,
            RoundA(Move::Scissors, Move::Paper) => 0,
            RoundA(Move::Scissors, Move::Scissors) => 3,
            RoundA(Move::Paper, Move::Rock) => 0,
            RoundA(Move::Paper, Move::Paper) => 3,
            RoundA(Move::Paper, Move::Scissors) => 6,
        };
        let move_points = match self {
            RoundA(_, Move::Rock) => 1,
            RoundA(_, Move::Paper) => 2,
            RoundA(_, Move::Scissors) => 3,
        };

        win_points + move_points
    }   
}


#[derive(Debug)]
struct RoundB(Move, DesiredResult);

impl RoundB
{
    fn points(&self) -> i32
    {
        RoundA(self.0, self.select_my_move()).points()
    }       

    fn select_my_move(&self) -> Move
    {
        match self {
            RoundB(Move::Rock, DesiredResult::Win) => Move::Paper,
            RoundB(Move::Rock, DesiredResult::Loose) => Move::Scissors,
            RoundB(Move::Rock, DesiredResult::Draw) => Move::Rock,
            RoundB(Move::Scissors, DesiredResult::Win) => Move::Rock,
            RoundB(Move::Scissors, DesiredResult::Loose) => Move::Paper,
            RoundB(Move::Scissors, DesiredResult::Draw) => Move::Scissors,
            RoundB(Move::Paper, DesiredResult::Win) => Move::Scissors,
            RoundB(Move::Paper, DesiredResult::Loose) => Move::Rock,
            RoundB(Move::Paper, DesiredResult::Draw) => Move::Paper,
        }
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();

    let rounds_a = lines.iter()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>())
        .map(|s| RoundA(s[0].parse::<Move>().unwrap(), s[1].parse::<Move>().unwrap()))
        .collect::<Vec<_>>();

    println!("(A) Rounds: {:?}", rounds_a);

    let points_a = rounds_a.iter().map(|r| r.points()).collect::<Vec<_>>();
    println!("(A) Points: {:?}", points_a);
    println!("(A) Sum of points: {}", points_a.iter().sum::<i32>());

    let rounds_b = lines.iter()
    .map(|l| l.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>())
    .map(|s| RoundB(s[0].parse::<Move>().unwrap(), s[1].parse::<DesiredResult>().unwrap()))
    .collect::<Vec<_>>();

    println!("(B) Rounds: {:?}", rounds_b);

    let points_b = rounds_b.iter().map(|r| r.points()).collect::<Vec<_>>();
    println!("(B) Points: {:?}", points_b);
    println!("(B) Sum of points: {}", points_b.iter().sum::<i32>());
}
