use std::{collections::HashMap, fmt::Display, cmp::min, cmp::max};
use std::io::{stdin, BufRead};

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
enum Tile {
    Sand,
    Rock,
    Air
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
        Ok(())
    }
}

#[derive(Debug,Clone)]
struct Map {
    tiles: HashMap<Point, Tile>,
    moving_sand: Point,
    top_left: Point,
    bottom_right: Point
}

impl Map {
    fn new() -> Map {
        Map {
            tiles: HashMap::new(),
            moving_sand: Point{x: 500, y: 0},
            top_left: Point{x: 0, y: 0},
            bottom_right: Point{x: 0, y: 0}
        }
    }

    fn get_at(&self, point: Point) -> Tile {
        if point == self.moving_sand {
            return Tile::Sand;
        }
        match self.tiles.get(&point) {
            Some(tile) => *tile,
            None => Tile::Air
        }
    }

    fn put_at(&mut self, point: Point, tile: Tile) {
        if self.tiles.is_empty() {
            self.top_left = Point{x: point.x - 1, y: point.y - 1 };
            self.bottom_right = Point{x: point.x + 1, y: point.y + 1};
        }

        self.top_left = Point{ 
            x: min(self.top_left.x, point.x),
            y: min(self.top_left.y, point.y)
        };

        self.bottom_right = Point{ 
            x: max(self.bottom_right.x, point.x),
            y: max(self.bottom_right.y, point.y)
        };

        self.tiles.insert(point, tile);
    }

    fn put_line(&mut self, start: Point, end: Point, tile: Tile) {
        match (start, end) {
            _ if start.x == end.x => {
                for y in min(start.y, end.y) ..= max(start.y, end.y) {
                    self.put_at(Point{x: start.x, y}, tile);
                }
            },
            _ if start.y == end.y => {
                for x in min(start.x, end.x) ..= max(start.x, end.x) {
                    self.put_at(Point{x, y: start.y}, tile);
                }
            }
            _ => panic!("Line from {} to {} is neither horizontal nor vertical!", start, end)
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.top_left.y - 2..=self.bottom_right.y {
            for x in self.top_left.x - 2..=self.bottom_right.x + 2 {
                f.write_str(match self.get_at(Point{x, y}) {
                    Tile::Sand => "o",
                    Tile::Rock => "#",
                    Tile::Air => ".",
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn simulate_a(mut map: Map)
{
    let mut units_at_rest = 0;
    loop {
        let (x, y) = (map.moving_sand.x, map.moving_sand.y);
        let candidate_points = vec![
            Point{x, y: y + 1},
            Point{x: x - 1, y: y + 1},
            Point{x: x + 1, y: y + 1}];
        let next_point = match candidate_points.iter().filter(|p| map.get_at(**p) == Tile::Air).next() {
            Some(p) => *p,
            None => {
                map.put_at(map.moving_sand, Tile::Sand);
                units_at_rest += 1;
                Point{x: 500, y: 0}                    
            },
        };
        map.moving_sand = next_point;
        if map.moving_sand.y > map.bottom_right.y {
            break;
        }
    }

    println!("After {} units of sand has fallen to rest, the rest fall into the abyss", units_at_rest);

}

fn simulate_b(mut map: Map)
{
    let mut units_at_rest = 0;
    let floor = map.bottom_right.y + 2;
    loop {
        if map.tiles.get(&Point{x: 500, y: 0})==Some(&Tile::Sand) 
        {
            break;
        }
        let (x, y) = (map.moving_sand.x, map.moving_sand.y);
        let candidate_points = vec![
            Point{x, y: y + 1},
            Point{x: x - 1, y: y + 1},
            Point{x: x + 1, y: y + 1}];
        let next_point = match candidate_points.iter().filter(|p| map.get_at(**p) == Tile::Air).next() {
            Some(p) if p.y < floor => *p,
            _ => {
                map.put_at(map.moving_sand, Tile::Sand);
                units_at_rest += 1;
                Point{x: 500, y: 0}                    
            },
        };
        map.moving_sand = next_point;
    }

    println!("After {} units of sand has fallen to rest, the source is blocked", units_at_rest);

}


fn main() {
    let lines = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty());

    let segments = lines
        .map(|l| l
                .split(" -> ")
                .map(|s| s.split(","))
                .map(|mut sp| 
                    Point{
                        x: sp.next().unwrap().parse::<i32>().unwrap(), 
                        y: sp.next().unwrap().parse::<i32>().unwrap()}
                    ).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map = Map::new();

    for line in segments {
        for line_segment in line.windows(2) {
            map.put_line(line_segment[0], line_segment[1], Tile::Rock);
        }
    }

    simulate_a(map.clone());
    simulate_b(map.clone());

}
