use std::io::{self, stdin, stdout, Write, BufRead};
use std::collections::BinaryHeap;
use std::fmt;
use std::rc::Rc;
use std::fs::File;
use std::env;
use std::cmp::Ordering;

#[derive(Copy,Clone,PartialEq,Debug)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

impl Amphipod {
    fn get_energy(&self, steps: usize) -> usize
    {
        match self {
            Amphipod::Amber => steps,
            Amphipod::Bronze => 10 * steps,
            Amphipod::Copper => 100 * steps,
            Amphipod::Desert => 1000 * steps
        }
    }
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
    Space,
    HomeSpace(Amphipod),
    Blank
}

#[derive(Clone)]
struct Board {
    rows: Rc<Vec<Vec<Block>>>,
    amphipods: Vec<AmphipodState>,
    height: usize,
    width: usize,
    max_home_pos: usize
}

#[derive(Copy,Clone,PartialEq,Debug)]
struct Pos {
    x: usize,
    y: usize
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy,Clone,Debug)]
struct AmphipodState {
    amphipod: Amphipod,
    pos: Pos
}

impl fmt::Display for AmphipodState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.amphipod, self.pos)
    }
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos{x, y}
    }

    fn up(&self) -> Pos {
        Pos{x: self.x, y: self.y - 1}
    }

    fn down(&self) -> Pos {
        Pos{x: self.x, y: self.y + 1}
    }

    fn left(&self) -> Pos {
        Pos{x: self.x - 1, y: self.y}
    }

    fn right(&self) -> Pos {
        Pos{x: self.x + 1, y: self.y}
    }

}

#[derive(Clone,Copy,Debug,PartialEq)]
struct Move {
    from: Pos,
    to: Pos,
    amphipod: Amphipod,
    energy: usize
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} -> {} ({} energy)", self.amphipod, self.from, self.to, self.energy)
    }
}

struct MoveIterator<'a> {
    board: &'a Board,
    amphipod_idx: usize,
    cur_pos: Pos
}

#[derive(Clone)]
struct BfsQueueItem {
    board: Board,
    moves: Vec<Move>,
    energy: usize
}

#[derive(Clone)]
struct PQueueItem {
    board: Board,
    moves: Vec<Move>,
    energy: usize
}

impl Eq for PQueueItem {

}

impl PartialEq for PQueueItem {
    fn eq(&self, other: &Self) -> bool {
        other.energy.eq(&self.energy)
    }
}

impl Ord for PQueueItem {
    // We want tominimze energy
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl PartialOrd for PQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl MoveIterator<'_> {
    fn new<'a>(board: &'a Board) -> MoveIterator<'a> {
        MoveIterator{
            board: board,
            amphipod_idx: 0,
            cur_pos: Pos::new(0, 0)
        }
    }
}

impl Iterator for MoveIterator<'_> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        for amphipod_idx in self.amphipod_idx..self.board.amphipods.len() {
            let amphipod_state = &self.board.amphipods[amphipod_idx];
            for y in self.cur_pos.y..self.board.height {
                for x in self.cur_pos.x..self.board.width {
                    let to = Pos::new(x, y);
                    if x >= self.board.rows[y].len() {
                        continue;
                    }
                    match self.board.is_legal_move(amphipod_state, &to) {
                        Some(steps) => {
                            self.cur_pos.x = x + 1;
                            self.cur_pos.y = y;
                            self.amphipod_idx = amphipod_idx;
                            return Some(Move{
                                from: amphipod_state.pos,
                                to,
                                amphipod: amphipod_state.amphipod,
                                energy: amphipod_state.amphipod.get_energy(steps)
                            });    
                        }
                        _ => ()                   
                    }
                }
                self.cur_pos.x = 0;
            }   
            self.cur_pos.y = 0; 
        }
        None
    }
}

impl Board {
    fn new(rows: &[Vec<(Block, Option<Amphipod>)>]) -> Board {
        let mut block_rows: Vec<Vec<Block>> = Vec::new();
        let mut amphipod_states: Vec<AmphipodState> = Vec::new();
        let mut max_home_pos = 0;

        for (y, row) in rows.iter().enumerate() {
            let mut board_row: Vec<Block> = Vec::new();
            for (x, (block, maybe_amphipod)) in row.iter().enumerate() {
                if let Some(amphipod) = maybe_amphipod {
                    amphipod_states.push(AmphipodState{
                        amphipod: *amphipod,
                        pos: Pos::new(x, y)
                    });
                } 

                if y >= 2 {
                    if let Block::Space = block {                
                        max_home_pos = y;
                        if x == 3 {
                            board_row.push(Block::HomeSpace(Amphipod::Amber));
                        } 
                        else if x == 5 {
                            board_row.push(Block::HomeSpace(Amphipod::Bronze));
                        } 
                        else if x == 7 {
                            board_row.push(Block::HomeSpace(Amphipod::Copper));
                        } 
                        else if x == 9 {
                            board_row.push(Block::HomeSpace(Amphipod::Desert));
                        } 
                    } else {
                        board_row.push(*block);
                    }
                }
                else {
                    board_row.push(*block);
                }
            }
            block_rows.push(board_row);
        }
        
        Board{
            rows: Rc::new(block_rows), 
            amphipods: amphipod_states,
            width: rows[0].len(),
            height: rows.len(),
            max_home_pos
        }
    }

    fn legal_moves<'a>(&'a self) -> MoveIterator<'a> {
        MoveIterator::new(self)
    }

    fn is_valid_target(&self, pos: &Pos) -> bool {
        if let Some(_) = self.amphipods.iter().filter(|a| a.pos == *pos).next() {
            return false;
        }
        match self.rows[pos.y][pos.x] {
            Block::Space => true,
            Block::HomeSpace(_) => true,
            _ => false
        }
    }

    fn has_clear_path(&self, from: &Pos, to: &Pos) -> Option<usize>
    {
        let mut pos = *from;
        let mut steps = 0;
        while pos.y > 1 {
            let new_pos = pos.up();
            if !self.is_valid_target(&new_pos) {
                break;
            }
            pos = new_pos;
            steps += 1;
        }
        while pos.x > to.x {
            let new_pos = pos.left();
            if !self.is_valid_target(&new_pos) {
                break;
            }
            pos = new_pos;
            steps += 1;
        }
        while pos.x < to.x {
            let new_pos = pos.right();
            if !self.is_valid_target(&new_pos) {
                break;
            }
            pos = new_pos;
            steps += 1;
        }
        while pos.y < to.y {
            let new_pos = pos.down();
            if !self.is_valid_target(&new_pos) {
                break;
            }
            pos = new_pos;
            steps += 1;
        }
        if pos == *to {
            Some(steps)
        } else {
            None
        }
    }


    fn can_stop(&self, pos: &Pos) -> bool {
        for y in (pos.y + 1)..self.height {
            if self.rows[y].len() <= pos.x {
                continue;
            }
            match self.rows[y][pos.x] {
                Block::HomeSpace(_) => { return false; }
                _ => ()
            }
        }
        true
    }

    fn is_home_ready(&self, amphipod: &Amphipod, pos: &Pos) -> bool {
        let x = match amphipod {
            Amphipod::Amber => 3,
            Amphipod::Bronze => 5,
            Amphipod::Copper => 7,
            Amphipod::Desert => 9
        };
        let amphipod_states: Vec<&AmphipodState> = self.amphipods.iter()
            .filter(|state| state.pos.x == x && state.pos.y >= 2)
            .collect();

        if !amphipod_states.iter().all(|a| a.amphipod == *amphipod) {
            return false;
        }

        let maybe_min_y = amphipod_states.iter().map(|state| state.pos.y).min();

        if let Some(min_y) = maybe_min_y {
            return pos.y == min_y -1;
        }

        pos.y == self.max_home_pos
    }

    fn is_legal_move(&self, amphipod_state: &AmphipodState, to: &Pos) -> Option<usize> {
        if to.x == amphipod_state.pos.x && to.y == amphipod_state.pos.y {
            return None;
        }

        let source_block = self.rows[amphipod_state.pos.y][amphipod_state.pos.x];
        
        match source_block {
            Block::HomeSpace(a) if a == amphipod_state.amphipod && 
                !self.amphipods.iter()
                    .any(|state| 
                        state.amphipod != amphipod_state.amphipod &&
                        (state.pos.y >= 2 && state.pos.y != amphipod_state.pos.y) &&
                        state.pos.x == amphipod_state.pos.x)=> { return None; }
            _ => ()
        };
        let is_source_in_corridor = if let Block::Space = source_block { true } else { false };
        let target_block = self.rows[to.y][to.x];
        match target_block {
            Block::Space if !is_source_in_corridor && self.can_stop(to) => self.has_clear_path(&amphipod_state.pos, to),
            Block::HomeSpace(a) if a == amphipod_state.amphipod && self.is_home_ready(&a, &to) => self.has_clear_path(&amphipod_state.pos, to),
            _ => None
        }
    }

    fn is_end_state(&self) -> bool {
        return self.amphipods.iter()
            .all(|state| match self.rows[state.pos.y][state.pos.x] {
                Block::HomeSpace(a) if a == state.amphipod => true,
                _ => false
            })
    }

    fn perform_move(&self, amphipod_move: &Move) -> Board {
        Board{
            width: self.width,
            height: self.height,
            amphipods: self.amphipods.iter()
                .map(|a| AmphipodState{
                    amphipod: a.amphipod,
                    pos: if a.pos == amphipod_move.from { amphipod_move.to } else { a.pos } 
                })
                .collect(),
            rows: self.rows.clone(),
            max_home_pos: self.max_home_pos
        }
    }

    fn solve_bh(&self) -> Option<(usize, Vec<Move>)> {
        let mut queue: BinaryHeap<PQueueItem> = BinaryHeap::new();
        queue.push(PQueueItem{board: self.clone(), moves: Vec::new(), energy: 0});
        let mut best_energy = usize::MAX;
        let mut best_moves: Vec<Move> = Vec::new();
        let mut num_moves = 0;
        let mut it_count = 0;
        while let Some(queue_item) = queue.pop() {
            it_count += 1;
            if queue_item.energy > best_energy {
                continue;
            }
            if queue_item.moves.len() > num_moves {
                num_moves = queue_item.moves.len();
                println!("Processing queue items with up to {} moves, queue size: {}", num_moves, queue.len());
            }

            let board = &queue_item.board;

            if board.is_end_state() {
                println!("Solution found: {} energy, {} moves", queue_item.energy, queue_item.moves.len());
                println!("Moves: ");
                for lm in queue_item.moves.iter() {
                    println!("{}", lm);
                }
                if queue_item.energy < best_energy {
                    best_energy = queue_item.energy;
                    best_moves = queue_item.moves.clone();
                    println!("This is the currently best solution.");
                }
                continue;
            }
            let mut legal_moves: Vec<Move> = board.legal_moves().collect();
            let home_moves: Vec<Move> = legal_moves.iter().filter(|m| match board.rows[m.to.y][m.to.x] {
                Block::HomeSpace(a) if a == m.amphipod => true,
                _ => false
            }).map(|m| *m).collect();
            if home_moves.len() > 0 {
                legal_moves = home_moves;
            }
            for legal_move in legal_moves.into_iter() {
                let new_board = board.perform_move(&legal_move);

                let mut moves = queue_item.moves.clone();
                moves.push(legal_move);
                queue.push(PQueueItem{board: new_board, moves: moves, energy: queue_item.energy + legal_move.energy});
            }
        }
        println!("Search terminated. Number of iterations: {}", it_count);
        if best_energy == usize::MAX {
            None
        } else {
            Some((best_energy, best_moves))
        }
    }


    fn solve_guided(&self) {
        let mut energy = 0;
        let mut board = self.clone();

        while !board.is_end_state() {
            println!("Used energy: {}", energy);
            println!("{}", board);
            println!("Available moves: ");
            let moves: Vec<Move> = board.legal_moves().collect();    

            for (i, legal_move) in moves.iter().enumerate() {
                println!("{}: {}", i, legal_move)
            }

            print!("Input move number: ");
            stdout().flush().expect("could not flush output");
            let n = get_input().trim().parse::<usize>().unwrap();

            board = board.perform_move(&moves[n]);
            energy += moves[n].energy;
        }

        println!("End state, used {} energy", energy);
        println!("{}", board);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.rows.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Some(amphipod_state) = self.amphipods.iter().filter(|a| a.pos.x == x && a.pos.y == y).next() {
                    amphipod_state.amphipod.fmt(f)?
                } else {
                    col.fmt(f)?
                }
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
            Block::Space => f.write_str(".")?,
            Block::HomeSpace(a) => f.write_str(&a.to_string().to_lowercase())?,
            Block::Blank => f.write_str(" ")?
        }
        Ok(())
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Unable to read input");
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut guided = false;

    if args.len() < 2 {
        println!("Expected file name");
        return;
    }
    let filename = &args[1];

    if args.len() >= 3 && args[2] == "--guided" {
        guided = true;
    }

    let file = File::open(filename).expect("Could not open file");
    
    let lineeiterator = io::BufReader::new(file).lines();

    let rows: Vec<Vec<(Block, Option<Amphipod>)>> = lineeiterator
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| match c {
                '#' => (Block::Wall, None),
                '.' => (Block::Space, None),
                'A' => (Block::Space, Some(Amphipod::Amber)),
                'B' => (Block::Space, Some(Amphipod::Bronze)),
                'C' => (Block::Space, Some(Amphipod::Copper)),
                'D' => (Block::Space, Some(Amphipod::Desert)),
                'a' => (Block::HomeSpace(Amphipod::Amber), None),
                'b' => (Block::HomeSpace(Amphipod::Bronze), None),
                'c' => (Block::HomeSpace(Amphipod::Copper), None),
                'd' => (Block::HomeSpace(Amphipod::Desert), None),
                _ => (Block::Blank, None)
            })
            .collect::<Vec<_>>()
        )
        .collect();

    let mut board = Board::new(&rows);
    println!("{}", board);

    if guided {
        board.solve_guided();
        for lmove in board.legal_moves() {
            println!("{}", lmove);
        }
        println!("{}", board.is_end_state());
        return;
    }

    let solution = board.solve_bh();
    if let Some((min_energy, mut moves)) = solution {
        println!("Minimum energy for move: {} using {} moves", min_energy, moves.len());
        while moves.len() > 0 {
            println!("{}", board);
            board = board.perform_move(&moves.remove(0));
        }
        println!("{}", board);
    } else {
        println!("No solution found.");
    }
}

