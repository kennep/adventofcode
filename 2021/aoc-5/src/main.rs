use std::io::{stdin,BufRead};
use std::cmp::{Ordering};

#[derive(Eq,Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            v => v
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point
}

fn coord_range(s: i32, e: i32) -> Box<dyn Iterator<Item=i32>> {
    if s > e {
        Box::new((e..=s).rev())
    } else {
        Box::new(s..=e)
    }
}

impl LineSegment {
    fn to_coords(&self) -> Vec<Point> {        
        if self.start.x == self.end.x {
            coord_range(self.start.y, self.end.y)
            .map(|y|
                Point{x: self.start.x, y: y}
            ).collect()
        } else if self.start.y == self.end.y {
            coord_range(self.start.x, self.end.x)
            .map(|x|
                Point{x: x, y: self.start.y}
            ).collect()
        } else {
            coord_range(self.start.x, self.end.x)
                .zip(coord_range(self.start.y, self.end.y))
                .map(|(x, y)|
                    Point{x: x, y:y}
                )
                .collect() 
        }
    }
}

fn parse_point(point: &str) -> Point
{
    let coords: Vec<&str> = point.split(",").collect();
    let x = coords[0].parse().expect(&format!("Expected number, not {}", coords[0]));
    let y = coords[1].parse().expect(&format!("Expected number, not {}", coords[1]));
    Point{x: x, y: y}
}

fn parse(line: &str) -> LineSegment {
    let mut segments = line.split(" -> ");
    let start_str = segments.next().unwrap();
    let end_str = segments.next().unwrap();
    let start = parse_point(&start_str);
    let end = parse_point(&end_str);
    LineSegment{ start: start, end: end }
}

fn two_or_more_overlap(coords: &[Point]) -> u32 {
    coords
    .iter()
    .fold((0, false, None), | (count, has_incremented, last_point), point | {
        match last_point {
            Some(p) if p == point =>
                (if has_incremented { count } else { count + 1 },
                true,
                Some(point)),
            _ => (count, false, Some(point))
        }
    }).0
}

fn main() {
    let input: Vec<LineSegment> = stdin().lock().lines()
    .map(|l| l.expect("failed to read line"))
    .filter(|l| l.len() > 0)
    .map(|l| parse(&l))
    .collect();

    let input_b: Vec<&LineSegment> = input.iter()
        .collect();

    let input_a: Vec<&LineSegment> = input.iter()
        .filter(|s| s.start.x == s.end.x || s.start.y == s.end.y)
        .collect();

    let mut coords_a: Vec<Point> = input_a.iter()
        .flat_map(|s| s.to_coords()).collect();
    coords_a.sort();

    println!("A: Dangerous points: {}", two_or_more_overlap(&coords_a));

    let mut coords_b: Vec<Point> = input_b.iter()
        .flat_map(|s| s.to_coords()).collect();
    coords_b.sort();
    
    println!("B: Dangerous points: {}", two_or_more_overlap(&coords_b));

}
