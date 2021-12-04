use std::io::{stdin, BufRead};
use std::fmt;

struct BoardElement {
    value: u32,
    marked: bool
}

struct Board {
    rows: Vec<Vec<BoardElement>>,
    won: bool
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            write!(f, "{}\n", row.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(" "))?
        }
        Ok(())
    }
}

impl fmt::Display for BoardElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, if self.marked { "X" } else { ""} )
    }
}

impl Board {
    fn mark(&mut self, num: u32) {
        for row in self.rows.iter_mut() {
            for elem in row.iter_mut() {
                if elem.value == num {
                    elem.marked = true;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        for row in self.rows.iter() {
            if row.iter().filter(|e| e.marked).count() == row.len() {
                return true;
            }
        }

        for colnum in 0..self.rows[0].len() {
            let col: Vec<&BoardElement> = self.rows.iter().map(|row| &row[colnum]).collect();
            if col.iter().filter(|e| e.marked).count() == col.len() {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let input: Vec<Vec<String>> = stdin().lock().lines()
    .map(|l| l.expect("failed to read line"))
    .collect::<Vec<String>>()
    .split(|l| l.len() == 0)
    .map(|lines| lines.to_vec())
    .collect();

    let drawn_numbers: Vec<u32> = input[0][0].split(",").map(|e| e.trim().parse().expect(&format!("Expected number, not {}", e))).collect();

    println!("{:?}", drawn_numbers);

    let mut boards:Vec<Board> =
        input.iter()
            .skip(1)
            .map(|lines| Board{
                won: false,
                rows: lines.iter()
                    .map(|line| line.split(" ").filter(|e| e.len() > 0).map(|e| BoardElement{ value: e.parse().expect(&format!("Expected number, not {}", e)), marked: false }).collect())
                    .collect()
            })
            .collect();

    for board in boards.iter() {
        println!("");
        println!("{}", board);
    }

    for draw in drawn_numbers {        
        for board in boards.iter_mut() {
            if board.won {
                continue;
            }
            board.mark(draw);
            if board.is_win() {
                println!("Drawn number: {} Winning board:\n{}", draw, board);
                let sum_unmarked = board.rows.iter().fold(0, |sum, row| sum + row.iter().fold(0, |sum, e| match e.marked {
                    true => sum,
                    false => sum + e.value
                }));
                println!("Sum of unmarked: {}", sum_unmarked);
                println!("Answer: {}\n", sum_unmarked * draw);
                board.won = true;
            }
        }
        if boards.iter().filter(|b| !b.won).count() == 0 {
            return;
        }
    }
}
