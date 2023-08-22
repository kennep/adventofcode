use std::io::{stdin, BufRead};
use std::cmp::max;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Tile {
    Rock,
    Air
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum JetDirection {
    Left,
    Right
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct Piece {
    tiles: Vec<Vec<Tile>>
}

impl Piece {
    fn new(tiles: &[&str]) -> Piece {
        Piece {
            tiles: tiles.iter()
                .map(|s| s.chars()
                    .map(|t| match t {
                        '#' => Tile::Rock,
                        _ => Tile::Air
                    })
                    .collect::<Vec<_>>())
                .collect()
        }
    }

    fn width(&self) -> i32 {
        return self.tiles.iter().map(|l| l.len() as i32).max().unwrap();
    }

    fn height(&self) -> i32 {
        return self.tiles.len() as i32;
    }
}

fn collides(board: &Vec<Vec<Tile>>, rock: &Piece, x: i32, y: i32) -> bool
{
    if x < 0 {
        return true;
    }
    if x + rock.width() > board[0].len() as i32 {
        return true;
    }
    if y + rock.height() > board.len() as i32 {
        return true;
    }

    for dy in 0..rock.height() {
        for dx in 0..rock.width() {
            if rock.tiles[dy as usize][dx as usize] == Tile::Rock &&
                board[y as usize + dy as usize][x as usize + dx as usize] == Tile::Rock {
                    return true;
                }
        }
    }

    false
}

fn print_board_with_rock(board: &[Vec<Tile>], rock: &Piece, rx: i32, ry: i32)
{
    println!("");
    for y in 0..board.len() as i32 {
        for x in 0..board[0].len() as i32 {
            if x >= rx && x < rx + rock.width()
                && y >= ry && y < ry + rock.height() &&
                rock.tiles[y as usize - ry as usize][x as usize - rx as usize] == Tile::Rock {
                    print!("@");
            } else {
                match board[y as usize][x as usize] {
                    Tile::Rock => print!("#"),
                    Tile::Air => print!(".")                    
                };
            }
        }
        println!("");
    }
}

fn find_first_row_with_stones(board: &Vec<Vec<Tile>>) -> i32
{
    let mut first_row_with_stones: i32 = 0;
    for y in 0..board.len() {
        if board[y].iter().any(|t| *t == Tile::Rock) {
            first_row_with_stones = y as i32;
            break;
        }
    }
    return first_row_with_stones;
}

struct BoardState {
    board: Vec<Vec<Tile>>,
    rock_count: i32,
    piece_idx: usize,
}

struct SimulateResult {
    height: i32,
    cycle_start: i32,
    cycle_height: i32,
    cycle_rocks: i32
}

fn simulate(pieces: &Vec<Piece>, jets: &Vec<JetDirection>, num_rocks: i32) -> SimulateResult
{
    let width = 7;
    let mut board: Vec<Vec<Tile>> = Vec::new();
    let mut next_piece_feed = pieces.iter().enumerate().cycle(); 
    let mut jet_feed = jets.iter().cycle();
    let mut fallen_rocks = 0;
    let mut rock_x: i32;
    let mut rock_y: i32;
    let mut board_states: Vec<BoardState> = Vec::new();
    let mut cycle_rocks: i32 = 0;
    let mut cycle_height: i32 = 0;
    let mut cycle_start: i32 = 0;
    while fallen_rocks < num_rocks {
        let (piece_idx, rock) = next_piece_feed.next().unwrap();
        let first_row_with_stones = find_first_row_with_stones(&board);

        //println!("First row with stones: {}", first_row_with_stones);
        for _ in 0..(3 + rock.height() - first_row_with_stones) {
            board.insert(0, std::iter::repeat(Tile::Air).take(width).collect::<Vec<_>>());
        }
        rock_x = 2;
        rock_y = max(0, first_row_with_stones - 3 - rock.height());
        loop {
            //print_board_with_rock(&board, rock, rock_x, rock_y);
            let jet_direction = jet_feed.next().unwrap();
            let maybe_x = match jet_direction {
                JetDirection::Left => rock_x - 1,
                JetDirection::Right => rock_x + 1,
            };
            if !collides(&board, rock, maybe_x, rock_y) {
                rock_x = maybe_x;
            }
            let maybe_y = rock_y + 1;
            if !collides(&board, rock, rock_x, maybe_y) {
                rock_y = maybe_y;
            } else {
                fallen_rocks += 1;
                break;
            }
        }
        //println!("Tile: {}x{} at {},{}", rock.width(), rock.height(), rock_x, rock_y);
        for y in 0..rock.height() {
            for x in 0..rock.width() {
                if rock.tiles[y as usize][x as usize] == Tile::Rock {
                    board[rock_y as usize + y as usize][rock_x as usize + x as usize] = Tile::Rock;
                } 
            }
        }

        if cycle_start == 0 {
            for board_state in board_states.iter().rev()
            {
                if board_state.board.len() < 50 {
                    continue;
                }
                if board_state.piece_idx != piece_idx {
                    continue;
                }
    
                let frws = find_first_row_with_stones(&board) as usize;
                let hsfr = find_first_row_with_stones(&board_state.board) as usize;
                if frws != hsfr {
                    continue;
                }
                for sy in 50..board.len() / 2 {
                    if frws + sy > board_state.board.len() {
                        break;
                    }
                    let historical_slice = &board_state.board[frws..frws + sy];
                    let now_slice = &board[frws..frws+sy];
                    if historical_slice == now_slice {
                        /* 
                        println!("frws={} hl={} nl={}", frws, board_state.board.len(), board.len());
                        println!("hrc={} nrc={}", board_state.rock_count, fallen_rocks);
                        print_board_with_rock(&historical_slice, &pieces[0], -100, -100);
                        println!("");
                        print_board_with_rock(&board_state.board, &pieces[0], -100, -100);
                        println!("---");
                        print_board_with_rock(&now_slice, &pieces[0], -100, -100);
                        println!("");
                        print_board_with_rock(&board, &pieces[0], -100, -100);
                        println!("Cycle found at size {}, piece {}", sy, piece_idx);
                        */
                        cycle_start = board_state.rock_count;
                        cycle_height = board.len() as i32 - board_state.board.len() as i32; 
                        cycle_rocks = fallen_rocks - board_state.rock_count;
                        break;
                    }
                }
                if cycle_start != 0 {
                    break;
                }
            }    
        }

        board_states.push(
            BoardState { board: board.clone(), piece_idx: piece_idx, rock_count: fallen_rocks }
        );
        //print_board_with_rock(&board, rock, -100, rock_y);
    }

    let first_row_with_stones = find_first_row_with_stones(&board);

    //print_board_with_rock(&board, &pieces[0], -100, -100);
    let height = board.len() as i32 - first_row_with_stones;
    println!("Tower of rocks is {} blocks high", board.len() as i32 - first_row_with_stones);
    println!("Cycle: start {} height {} num. rocks: {}", cycle_start, cycle_height, cycle_rocks);

    SimulateResult { height: height, cycle_start: cycle_start, cycle_height: cycle_height, cycle_rocks: cycle_rocks }
}

fn main() {
    let pieces: Vec<Piece> = vec![
        Piece::new(&vec!["####"]),
        Piece::new(&vec![".#.",
                         "###",
                         ".#."]),
        Piece::new(&vec!["..#",
                         "..#",
                         "###"]),
        Piece::new(&vec!["#",
                         "#",
                         "#",
                         "#"]),
        Piece::new(&vec!["##",
                         "##"])
    ];

    let jets = stdin().lock().lines().next().unwrap().unwrap()
        .chars().map(|c| match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => panic!("Unknown input: {}", c)
        }).collect::<Vec<_>>();

    let result = simulate(&pieces, &jets, 2022);

    let total_rocks: i64 = 1000000000000;
    let rocks_after_cycle_start = total_rocks - result.cycle_start as i64;
    let cycles = rocks_after_cycle_start / result.cycle_rocks as i64;
    let rocks_covered_by_cycles = cycles * result.cycle_rocks as i64;
    let rocks_after_cycles = rocks_after_cycle_start - rocks_covered_by_cycles;
    let rocks_to_simulate = result.cycle_start + rocks_after_cycles as i32;

    let result2 = simulate(&pieces, &jets, rocks_to_simulate);

    let total_result = result2.height as i64 + cycles * result.cycle_height as i64;
    println!("Height after simulating {} falling rocks: {}", total_rocks, total_result);
}
