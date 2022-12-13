use std::collections::VecDeque;
use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Copy)]
struct Block {
    elevation: i32,
    visited: bool,
    is_start: bool,
    is_end: bool,
    cost: i32,
}

impl Block {
    fn new(input: char) -> Block {
        match input {
            'a'..='z' => Block {
                elevation: input as i32 - 'a' as i32,
                visited: false,
                is_start: false,
                is_end: false,
                cost: 0,
            },
            'S' => Block {
                elevation: 0 as i32,
                visited: false,
                is_start: true,
                is_end: false,
                cost: 0,
            },
            'E' => Block {
                elevation: 'z' as i32 - 'a' as i32,
                visited: false,
                is_start: false,
                is_end: true,
                cost: 0,
            },
            _ => panic!("Unexpected input {}", input),
        }
    }
}

trait Map {
    fn get_block<'a>(&'a self, pos: Point) -> &'a Block;
    fn get_block_mut<'a>(&'a mut self, pos: Point) -> &'a mut Block;
    fn edges(&self, pos: Point) -> Vec<Point>;
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Map for Vec<Vec<Block>> {
    fn get_block<'a>(&'a self, pos: Point) -> &'a Block {
        return &self[pos.y as usize][pos.x as usize];
    }

    fn get_block_mut<'a>(&'a mut self, pos: Point) -> &'a mut Block {
        let line: &mut Vec<Block> = self.get_mut(pos.y as usize).unwrap();
        return line.get_mut(pos.x as usize).unwrap();
    }

    fn edges(&self, pos: Point) -> Vec<Point> {
        let positions = vec![pos.up(), pos.down(), pos.left(), pos.right()];
        let elevation = self.get_block(pos).elevation;
        return positions
            .into_iter()
            .filter(|p| {
                p.x >= 0 && p.x < self[0].len() as i32 && p.y >= 0 && p.y < self.len() as i32
            })
            .filter(|p| (self.get_block(*p).elevation - elevation) <= 1)
            .collect();
    }
}

fn bfs(mut map: Vec<Vec<Block>>, start: Point) -> i32 {
    let mut q: VecDeque<Point> = VecDeque::new();
    let start_pos: &mut Block = map.get_block_mut(start);
    start_pos.visited = true;
    q.push_back(start);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        let cur_block_cost;
        {
            let cur_block = map.get_block(v);
            cur_block_cost = cur_block.cost;
            if cur_block.is_end {
                return cur_block.cost;
            }
        }
        for edge in map.edges(v) {
            let edge_block: &mut Block = map.get_block_mut(edge);
            if !edge_block.visited {
                edge_block.visited = true;
                edge_block.cost = cur_block_cost + 1;
                q.push_back(edge);
            }
        }
    }
    i32::MAX
}

fn find_start(map: &Vec<Vec<Block>>) -> Point {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x].is_start {
                return Point {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }
    panic!("No start block");
}

fn main() {
    let map: Vec<Vec<Block>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| Block::new(c)).collect())
        .collect();

    let start = find_start(&map);
    let steps = bfs(map.clone(), start);
    println!("Solution found in {} steps", steps);

    let mut step_solutions: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(move |(y, r)| r.iter().enumerate().map(move |(x, b)| (x, y, b.elevation)))
        .filter(|(_, _, e)| *e == 0)
        .map(|(x, y, _)| Point {
            x: x as i32,
            y: y as i32,
        })
        .map(|p| bfs(map.clone(), p))
        .collect();

    step_solutions.sort();

    println!(
        "Minimum number of steps from a starting point: {}",
        step_solutions[0]
    );
}
