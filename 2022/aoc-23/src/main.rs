use std::io::{stdin, BufRead};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Write, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Elf {
    position: Position,
}

#[derive(Debug, Clone)]
struct ElfMap {
    elves: HashMap<Position, Elf>
}

impl ElfMap {
    fn new<E: Iterator<Item=Elf>>(elves: E) -> ElfMap {
        ElfMap{elves: HashMap::from_iter(elves.map(|e| (e.position, e)))}
    }

    fn bounding_rect(&self) -> (Position, Position) {
        self.elves.values()
            .fold((Position{x: i32::MAX, y: i32::MAX}, Position{x: i32::MIN, y: i32::MIN}), 
                | (tl, br), e|
                    (Position{x: min(tl.x, e.position.x), y: min(tl.y, e.position.y)},
                     Position{x: max(br.x, e.position.x), y: max(br.y, e.position.y)}))                
    }

    fn empty_positions(&self) -> usize {
        let (tl, br) = self.bounding_rect();
        (tl.y..=br.y).flat_map(|y| (tl.x..=br.x).map(move |x| (x, y)))
            .filter(|(x, y)| !self.elves.contains_key(&Position{x: *x, y: *y}))
            .count()
    }
}

impl Display for ElfMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tl, br) = self.bounding_rect();
        for y in tl.y..=br.y {
            for x in tl.x..=br.x {
                if self.elves.contains_key(&Position{x, y}) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
//                                          NW: 0     N: 1     NE: 2    W: 3     E: 4    SW: 5    S: 6    SE: 7
const ADJACENT_POSITIONS: [(i32, i32);8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]; 

fn propose_direction<'a, D>(elf: &Elf, elves: &ElfMap, directions: D) -> Option<Position>
    where D: Iterator<Item=&'a Direction>
{
    let adjacent_positions: Vec<_> = ADJACENT_POSITIONS.iter()
        .map(|&(x, y)| elves.elves.contains_key(&Position{x: elf.position.x + x, y: elf.position.y + y}))
        .collect();
    if adjacent_positions.iter().all(|p| *p == false) {
        return None
    }
    //println!("ELF {:?}, adjacent: {:?}", elf.position, adjacent_positions);
    directions.map(|&direction| {
        match direction {
            Direction::North => if !adjacent_positions[0] && !adjacent_positions[1] && !adjacent_positions[2] { Some(Position{x: elf.position.x, y: elf.position.y - 1}) } else { None }
            Direction::South => if !adjacent_positions[5] && !adjacent_positions[6] && !adjacent_positions[7] { Some(Position{x: elf.position.x, y: elf.position.y + 1}) } else { None },
            Direction::West => if !adjacent_positions[0] && !adjacent_positions[3] && !adjacent_positions[5] { Some(Position{x: elf.position.x - 1, y: elf.position.y}) } else { None },
            Direction::East => if !adjacent_positions[2] && !adjacent_positions[4] && !adjacent_positions[7] { Some(Position{x: elf.position.x + 1, y: elf.position.y}) } else { None },
        }
    })
    .filter(|p| match p { Some(_) => true, None => false})
    .nth(0).unwrap_or(None)
}

fn part_a(mut elves: ElfMap)
{
    let directions = vec![Direction::North, Direction::South, Direction::West, Direction::East];
    let mut start_direction = directions.iter().cycle();

    println!("{}", elves);
    for round in 0..10 {
        let mut proposed_positions: HashMap<Position, Vec<Position>> = HashMap::new();
        let directions: Vec<_> = start_direction.clone().take(4).collect();
        println!("{}", round + 1);
        for elf in elves.elves.values() {
            let maybe_proposed_position = propose_direction(elf, &elves, directions.clone().into_iter());
            if let Some(proposed_position) = maybe_proposed_position {
                //println!("Elf at {:?} proposes moving to {:?}", elf.position, proposed_position);
                proposed_positions.entry(proposed_position).or_insert(Vec::new()).push(elf.position);
            } /*else {
                println!("Elf at {:?} does not move", elf.position);
            }*/
        }
        for (position, proposing_elves_pos) in proposed_positions.iter() {
            if proposing_elves_pos.len() == 1 {
                let mut elf = elves.elves.remove(&proposing_elves_pos[0]).unwrap();
                elf.position = *position;
                elves.elves.insert(*position, elf);
            }
        }        
        start_direction.next();
        println!("{}", elves);
    }
    println!("Number of empty positions: {}", elves.empty_positions());
}

fn part_b(mut elves: ElfMap)
{
    let directions = vec![Direction::North, Direction::South, Direction::West, Direction::East];
    let mut start_direction = directions.iter().cycle();

    let mut rounds = 0;
    loop {
        let mut proposed_positions: HashMap<Position, Vec<Position>> = HashMap::new();
        let mut moves = 0;
        let directions: Vec<_> = start_direction.clone().take(4).collect();
        println!("{}", rounds + 1);
        for elf in elves.elves.values() {
            let maybe_proposed_position = propose_direction(elf, &elves, directions.clone().into_iter());
            if let Some(proposed_position) = maybe_proposed_position {
                //println!("Elf at {:?} proposes moving to {:?}", elf.position, proposed_position);
                proposed_positions.entry(proposed_position).or_insert(Vec::new()).push(elf.position);
            } /*else {
                println!("Elf at {:?} does not move", elf.position);
            }*/
        }
        for (position, proposing_elves_pos) in proposed_positions.iter() {
            if proposing_elves_pos.len() == 1 {
                let mut elf = elves.elves.remove(&proposing_elves_pos[0]).unwrap();
                elf.position = *position;
                elves.elves.insert(*position, elf);
                moves += 1;
            }
        }        
        start_direction.next();
        println!("{}", elves);
        rounds += 1;
        if moves == 0 {
            break;
        }
    }
    println!("Number of rounds until no elf moves: {}", rounds);
}

fn main() {
    let map = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(|(x, c)| (x, y, c)).collect::<Vec<_>>() )
        .filter(|(_, _, c)| *c == '#')
        .map(|(x, y, _)| Position{x: x as i32, y: y as i32});

    let elves = ElfMap::new(map.map(|p| Elf{ position: p }));
    
    part_a(elves.clone());
    part_b(elves.clone());
}
