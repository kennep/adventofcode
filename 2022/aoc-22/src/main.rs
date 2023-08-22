use std::io::{stdin, BufRead};
use regex::Regex;

#[derive(Debug,Copy,Clone,PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    OffMap
}

#[derive(Debug,Copy,Clone)]
enum Instruction {
    Move(i32),
    TurnRight,
    TurnLeft
}

#[derive(Debug,Copy,Clone)]
enum Facing {
    Left,
    Right,
    Up,
    Down
}

impl Facing {
    fn turn_left(&self) -> Facing {
        match self {
            Facing::Left => Facing::Down,
            Facing::Right => Facing::Up,
            Facing::Up => Facing::Left,
            Facing::Down => Facing::Right,
        }
    }
    
    fn turn_right(&self) -> Facing {
        match self {
            Facing::Left => Facing::Up,
            Facing::Right => Facing::Down,
            Facing::Up => Facing::Right,
            Facing::Down => Facing::Left,
        }
    }

    fn face_val(&self) -> i32 {
        match self {
            Facing::Left => 2,
            Facing::Right => 0,
            Facing::Up => 3,
            Facing::Down => 1,
        }        
    }
}

fn next_pos(map: &Vec<Vec<Tile>>, x: i32, y: i32, facing: Facing) -> (i32, i32)
{
    let (dx, dy) = match facing {
        Facing::Left => (-1, 0),
        Facing::Right => (1, 0),
        Facing::Up => (0, -1),
        Facing::Down => (0, 1)
    };
    let mut px = x;
    let mut py = y;
    loop {
        px += dx;
        py += dy;
        if py >= map.len() as i32 {
            py = 0;
        }
        if py < 0 {
            py = (map.len() - 1) as i32;
        }
        if px >= map[0].len() as i32 {
            px = 0;
        }
        if px < 0 {
            px = (map[0].len() - 1) as i32;
        }
        if px >= map[py as usize].len() as i32 {
            continue;
        }

        match map[py as usize][px as usize] {
            Tile::Wall => return (x, y),
            Tile::Open => return (px, py),
            _ => continue
        }
    }
}


fn next_pos_2(map: &Vec<Vec<Tile>>, x: i32, y: i32, facing: Facing) -> (i32, i32, Facing)
{
    let (dx, dy) = match facing {
        Facing::Left => (-1, 0),
        Facing::Right => (1, 0),
        Facing::Up => (0, -1),
        Facing::Down => (0, 1)
    };
    let mut px = x;
    let mut py = y;
    let mut pfacing = facing;
    loop {
        px += dx;
        py += dy;
        (px, py, pfacing) = warp_big(px, py, pfacing);

        let tile = match (px, py) {
            _ if py >= 0 && py < map.len() as i32 && px >= 0 && px < map[py as usize].len() as i32 => {
                map[py as usize][px as usize]
            },
            _ => {
                //println!("{},{} is off map", px, py);
                Tile::OffMap
            }
        };

        match tile {
            Tile::Wall => return (x, y, facing),
            Tile::Open => return (px, py, pfacing),
            _ => panic!("Unexpected coordinates: {},{} {:?} -> {}, {} {:?}", x, y, facing, px, py, pfacing)
        }
    }
}

// Note: specific to the particular shape of the input data
fn warp_big(x: i32, y: i32, facing: Facing) -> (i32, i32, Facing)
{
    let rv = match (x, y, facing) {
        _ if x >= 50 && x < 100 && y == -1 => {
            // up 1 to side 6
            (0, x - 50 + 150, Facing::Right)
        },
        _ if y >= 0 && y < 50 && x == 49 => {
            // left 1 to side 5 (upside down)
            (0, 49 - y + 100, Facing::Right)
        },
        _ if x >= 100 && x < 150 && y == -1 => {
            // up 2 to bottom 6
            (x - 100, 199, Facing::Up)
        },
        _ if y >= 0 && y < 50 && x == 150 => {
            // right 2 to right 4 (upside down)
            (99, 149 - y, Facing::Left)
        },
        _ if x >= 100 && x < 150 && y == 50 => {
            // bottom 2 to right 3
            (99, x - 100 + 50, Facing::Left)
        },
        _ if y >= 50 && y < 100 && x == 49 => {
            // left 3 to top 5
            (y - 50, 100, Facing::Down)
        },
        _ if y >= 50 && y < 100 && x == 100 => {
            // right 3 to bottom 2
            (y - 50 + 100, 49, Facing::Up)
        },
        _ if y >= 100 && y < 150 && x == 100 => {
            // right 4 to right 2 (upside down)
            (149, 49 - (y - 100), Facing::Left)
        },
        _ if x >= 50 && x < 100 && y == 150 => {
            // bottom 4 to right 6
            (49, x - 50 + 150, Facing::Left)
        },
        _ if x >= 0 && x < 50 && y == 99 => {
            // top 5 to left 3
            (50, x + 50, Facing::Right)
        },
        _ if y >= 100 && y < 150 && x == -1 => {
            // side 5 to left 1 (upside down)
            (50, 49 - (y - 100), Facing::Right)
        },
        _ if y >=150 && y < 200 && x == -1 => {
            // left 6 to top 1
            (y - 150 + 50, 0, Facing::Down)
        },
        _ if y >= 150 && y < 200 && x == 50 => {
            // right 6 to bottom 4
            (y - 150 + 50, 149, Facing::Up)
        },
        _ if x >= 0 && x < 50 && y == 200 => {
            // bottom 6 to top 2
            (x + 100, 0, Facing::Down)
        },
        _ => (x, y, facing)
    };
    rv
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .collect();

    let mut map_and_path = lines.split(|l| l.is_empty());

    let map: Vec<Vec<Tile>> = map_and_path.next().unwrap().iter()
        .map(|l| l.chars().map(|c| match c {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => Tile::OffMap
        }).collect())
        .collect();

    let path_regex = Regex::new(r"(\d+)|R|L").unwrap();
    let path_str = &map_and_path.next().unwrap()[0];

    let path: Vec<_> = path_regex.captures_iter(path_str)
        .map(|c| c.get(0).unwrap().as_str() )
        .map(|s| match s {
            "R" => Instruction::TurnRight,
            "L" => Instruction::TurnLeft,
            _ => Instruction::Move(s.parse::<i32>().unwrap())
        })
        .collect();

    println!("Path: {:?}", path);

    let mut y = 0;
    let mut x = map[0].iter().position(|t| *t == Tile::Open).unwrap() as i32;
    let mut facing = Facing::Right;

    println!("start: {}, {}", x, y);

    for instruction in path.iter() {
        match *instruction {
            Instruction::Move(d) => for _ in 0..d {
                (x, y) = next_pos(&map, x, y, facing);
            },
            Instruction::TurnLeft => facing = facing.turn_left(),
            Instruction::TurnRight => facing = facing.turn_right()
        };
        //println!("{:?} Move to {},{} {:?}", instruction, x, y, facing);
    }

    println!("Password: {}", 1000*(y+1)+4*(x+1)+facing.face_val());

    if map.len() < 100 {
        panic!("Don't know how to compute second part for example data");
    }

    let mut y = 0;
    let mut x = map[0].iter().position(|t| *t == Tile::Open).unwrap() as i32;
    let mut facing = Facing::Right;
    for instruction in path.iter() {
        match *instruction {
            Instruction::Move(d) => for _ in 0..d {
                (x, y, facing) = next_pos_2(&map, x, y, facing);
            },
            Instruction::TurnLeft => facing = facing.turn_left(),
            Instruction::TurnRight => facing = facing.turn_right()
        };
        //println!("{:?} Move to {},{} {:?}", instruction, x, y, facing);
    }

    println!("Password: {}", 1000*(y+1)+4*(x+1)+facing.face_val());

}
