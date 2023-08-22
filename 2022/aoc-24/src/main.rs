use std::{io::{stdin, BufRead}, fmt::Display, fmt::Write, cmp::min};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn transpose(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => (self.x, self.y - 1).into(),
            Direction::Down => (self.x, self.y + 1).into(),
            Direction::Left => (self.x - 1, self.y).into(),
            Direction::Right => (self.x + 1, self.y).into(),
        }
    }

    fn mh_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        return Position{x, y}
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        return Position{x: x as i32, y: y as i32}
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    position: Position
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Step(Direction, Position, i32),
    Wait(Position, i32)
}

impl Move {
    fn position(&self) -> &Position {
        match self {
            Move::Step(_, pos, _) => pos,
            Move::Wait(pos, _) => pos,
        }
    }

    fn goal_distance(&self) -> i32 {
        match self {
            Move::Step(_, _, d) => *d,
            Move::Wait(_, d) => *d,
        }
    }
}

impl Blizzard {
    fn with_x(&self, x: i32) -> Blizzard {
        Blizzard { direction: self.direction, position: (x, self.position.y).into() }
    }

    fn with_y(&self, y: i32) -> Blizzard {
        Blizzard { direction: self.direction, position: (self.position.x, y).into() }
    }

    fn with_position(&self, position: Position) -> Blizzard {
        Blizzard { direction: self.direction, position: position }
    }
}

#[derive(Debug, Clone)]
struct Map {
    start: Position,
    goal: Position,
    blizzards: Vec<Blizzard>,
    player: Position,
    width: i32
}

impl Map {
    fn is_boundary(&self, position: &Position) -> bool {
        position.y == self.start.y ||
            position.y == self.goal.y ||
            position.x == 0 ||
            position.x == self.width + 1
    }

    fn is_wall(&self, position: &Position) -> bool {
        self.is_boundary(position)
            && *position != self.goal
    }

    fn get_moved_blizzards(&self, blizzards: &[Blizzard]) -> Vec<Blizzard> {
        blizzards.iter().map(|blizzard|  {
            let new_pos = blizzard.position.transpose(&blizzard.direction);
            if self.is_boundary(&new_pos) {
                match &blizzard.direction {
                    Direction::Up => blizzard.with_y(self.goal.y - 1),
                    Direction::Down => blizzard.with_y(1),
                    Direction::Left => blizzard.with_x(self.width),
                    Direction::Right => blizzard.with_x(1),
                }
            } else {
                blizzard.with_position(new_pos)
            }   
        }).collect()
    } 

    fn get_valid_moves(&self, position: &Position, blizzards: &[Blizzard]) -> Vec<Move> {
        vec![Direction::Right, Direction::Down, Direction::Left, Direction::Up].iter().map(|direction| {
            let direction_pos = position.transpose(direction);
            if direction_pos.y > 0 && !self.is_wall(&direction_pos) && !blizzards.iter().any(|b| b.position == direction_pos) {
                Some(Move::Step(*direction, direction_pos, direction_pos.mh_distance(&self.goal)))
            } else {
                None
            }
        })
        .chain(std::iter::once(if !blizzards.iter().any(|b| b.position == *position) { Some(Move::Wait(position.clone(), position.mh_distance(&self.goal))) } else { None }))
        .filter(|m| m.is_some() )
        .map(|m| m.unwrap() )
        .collect()
    }

    fn move_blizzards(&mut self)
    {
        self.blizzards = self.get_moved_blizzards(&self.blizzards);
    }

    fn best_move_for_pos_and_blizzards(&self, position: &Position, blizzards: &[Blizzard], planning_horizon: usize) -> Option<(Move, i32)>
    {
        let mut moves = self.get_valid_moves(position, blizzards);
        if moves.len() == 0 {
            return None
        }
        if planning_horizon == 1 {
            moves.sort_by(|a, b| a.goal_distance().cmp(&b.goal_distance()));
            return Some((moves[0], moves[0].position().mh_distance(&self.goal)));
        } else {
            let blizzards_next = self.get_moved_blizzards(blizzards);
            let mut next_moves: Vec<(Move, i32)> = moves.iter().map(|m| {
                (m, self.best_move_for_pos_and_blizzards(m.position(), &blizzards_next, planning_horizon - 1))
            })
            .map(|(m, n)| match n {
                Some((_, d)) => {
                    if(d < m.goal_distance()) {
                        //println!("{} Improved d {}  -> {}", planning_horizon, m.goal_distance(), d);
                    }
                    (*m, min(d, m.goal_distance()))
                },
                None => (*m, m.goal_distance())
            }).collect();
            next_moves.sort_by(|(_, da), (_, db)| da.cmp(db));
            return Some((next_moves[0].0, next_moves[0].1))
        }
    }

    fn best_move(&self, planning_horizon: usize) -> Option<Move> {
        match self.best_move_for_pos_and_blizzards(&self.player, &self.blizzards, planning_horizon) {
            Some((m, _)) => Some(m),
            None => None
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.goal.y {
            for x in 0..=(self.width + 1) {
                match (Position{x, y}) {
                    p if p == self.player => f.write_char('X')?,
                    p if p == self.start => f.write_char('.')?,
                    p if p == self.goal => f.write_char('.')?,
                    _ if y == 0 => f.write_char('#')?,
                    _ if y == self.goal.y => f.write_char('#')?,
                    _ if x == 0 => f.write_char('#')?,
                    _ if x == self.width + 1 => f.write_char('#')?,
                    p => {
                        let blizzards: Vec<_> = self.blizzards.iter()
                            .filter(|b| b.position == p)
                            .collect();
                        match blizzards.len() {
                            0 => f.write_char('.')?,
                            1 => f.write_char(match blizzards[0].direction {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                                Direction::Right => '>',
                            })?,
                            n => f.write_char(n.to_string().chars().nth(0).unwrap())?
                        }

                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn main() {
    let map = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .enumerate()
        .fold(Map{ start: (0, 0).into(), goal: (0, 0).into(), blizzards: Vec::new(), player: (0, 0).into(), width: 0 },
        |mut map, (y, l)| {
            if l.chars().filter(|c| *c == '#').count() > 2 {
                // start or end line
                let open_pos = l.find('.').unwrap();
                if y == 0 {
                    map.start = (open_pos, y).into();
                    map.player = map.start.clone();
                    map.width = (l.len() - 2) as i32;
                } else {
                    map.goal = (open_pos, y).into();
                }
            }
            l.chars().enumerate().fold(map, |mut map, (x, c)| {
                let maybe_direction = match c {
                    '>' => Some(Direction::Right),
                    '<' => Some(Direction::Left),
                    'v' => Some(Direction::Down),
                    '^' => Some(Direction::Up),
                    _ => None
                };
                if let Some(direction) = maybe_direction {
                    map.blizzards.push(Blizzard{direction, position: (x, y).into()});
                }
                map
            })
        });

    println!("Map: {:?}", map);
    println!("Map:\n{}", map);

    let mut map = map.clone();
    let mut rounds = 0;
    loop {
        rounds+=1;
        let m = map.best_move(14);
        if let Some(m) = m {
            println!("{} {:?}:", rounds, m);
            map.move_blizzards();
            map.player = *m.position();
        } else {
            panic!("No valid moves found!");
        }
        println!("{}", map);
        if map.player == map.goal {
            println!("Goal reached after {} moves", rounds);
            break;
        }
    }
}
