use std::io::{stdin,BufRead};
use std::cmp::{min, max};

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Direction {
    fn offset(&self, origin: Point) -> Point
    {
        match self {
            Direction::Up(d) => Point::new(origin.x, origin.y + d),
            Direction::Down(d) => Point::new(origin.x, origin.y - d),
            Direction::Left(d) => Point::new(origin.x - d, origin.y),
            Direction::Right(d) => Point::new(origin.x + d, origin.y)
        }
    } 
}

#[derive(Debug, Copy,Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }

    fn mh_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Copy,Clone)]
struct Segment {
    start: Point,
    end: Point
}

impl Segment {
    fn new(start: Point, end: Point) -> Segment{
        Segment{start, end}
    }

    fn intersection(&self, other: &Segment) -> Option<Point> {
        if self.start.x == self.end.x && other.start.y == other.end.y {
            // We're vertical, other horizontal
            if (min(self.start.y, self.end.y) <= other.start.y && max(self.start.y, self.end.y) >= other.start.y) &&
               (min(other.start.x, other.end.x) <= self.start.x && max(other.start.x, other.end.x) >= self.start.x) {
                return Some(Point::new(self.start.x, other.start.y))
            }
        }
        if self.start.y == self.end.y && other.start.x == other.end.x {
            // We're horizontal, other vertical
            if (min(self.start.x, self.end.x) <= other.start.x && max(self.start.x, self.end.x) >= other.start.x) &&
               (min(other.start.y, other.end.y) <= self.start.y && max(other.start.y, other.end.y) >= self.start.y) {
                return Some(Point::new(other.start.x, self.start.y))
            }
        }
        None
    }

    fn mh_length(&self) -> i32 {
        self.start.mh_distance(&self.end)
    }

}

fn minimum<E: Ord>(it: impl Iterator<Item=E>) -> E {
    it.fold(None, |cur, next| {
        match cur {
            None => Some(next),
            Some(e) if next < e => Some(next),
            _ => cur
        }
    }).unwrap()
}

struct Wire {
    segments: Vec<Segment>
}

impl Wire {
    fn new() -> Wire {
        Wire { segments: Vec::new() }
    }

    fn parse_directions(input: &str) -> Wire {
        let segment_directions: Vec<Direction> = input
            .split(',')
            .map(|d| match (&d[0..1], d[1..].parse::<i32>().unwrap()) {
                ("U", d) => Direction::Up(d),
                ("D", d) => Direction::Down(d),
                ("L", d) => Direction::Left(d),
                ("R", d) => Direction::Right(d),
                _ => panic!("Invalid segment: {}", input)
            })
            .collect();
        let mut segments: Vec<Segment> = Vec::new();
        let mut start = Point::new(0, 0);
        for segment_direction in segment_directions {
            let end = segment_direction.offset(start);
            segments.push(Segment::new(start, end));
            start = end;
        }

        Wire{segments}
    }

    fn find_intersections(&self, other: &Wire) -> Vec<Point> {
        let mut intersections: Vec<Point> = Vec::new();
        for my_segment in &self.segments {
            for other_segment in &other.segments {
                match my_segment.intersection(&other_segment) {
                    Some(point) => {
                        intersections.push(point);
                    },
                    None => ()
                };
            }
        }
        intersections
    }

    fn find_intersection_length(&self, other: &Wire) -> i32 {
        let mut intersection_lengths: Vec<i32> = Vec::new();
        let mut my_length = 0;
        for my_segment in &self.segments {
            let mut other_length = 0;
            for other_segment in &other.segments {
                match my_segment.intersection(&other_segment) {
                    Some(point) if point.x > 0 && point.y > 0 => {
                        intersection_lengths.push(my_length + my_segment.start.mh_distance(&point) + other_length + other_segment.start.mh_distance(&point))
                    },
                    _ => ()
                };
                other_length += other_segment.mh_length();
            }
            my_length += my_segment.mh_length();
        }
        return *minimum(intersection_lengths.iter())
    }

}

fn main() {
    let input: Vec<Wire> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| Wire::parse_directions(&l))
        .collect();

    let mut min_distance: Option<i32> = None;
    for intersection in input[0].find_intersections(&input[1]) {
        let distance = intersection.x.abs() + intersection.y.abs();
        println!("Intersection: {:?} Manhattan distance: {}", intersection, distance);
        match min_distance {
            None if distance > 0 => { min_distance = Some(distance) },
            Some(d) if d > distance && distance > 0 => { min_distance = Some(distance) }
            _ => ()
        }
    }
    println!("Min distance: {}", min_distance.unwrap());
    println!("Min intersection length: {}", input[0].find_intersection_length(&input[1]))
}
