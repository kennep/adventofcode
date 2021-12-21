use std::io::{stdin,Read};
use std::fmt;
use std::collections::{HashSet,HashMap};
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ScannerParser;

#[derive(Copy,Clone,Eq,PartialEq,Hash,Ord,PartialOrd)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}",
            self.x, self.y, self.z
        )
    }
}

struct Scanner {
    name: String,
    coords: HashSet<Coord>
}

#[derive(Eq,PartialEq,Hash)]
enum Object {
    Beacon(Coord),
    Scanner(String, Coord)
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--- scanner {} ---\n{}",
            self.name,
            self.coords.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n")
        )
    }
}

struct Scanners {
    elems: Vec<Scanner>
}

impl fmt::Display for Scanners {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            self.elems.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n\n")
        )
    }
}

fn parse_coord(input: Pair<Rule>) -> Coord {
    let mut coord_it = input.into_inner();
    Coord{
        x: coord_it.next().unwrap().as_str().parse::<i32>().unwrap(), 
        y: coord_it.next().unwrap().as_str().parse::<i32>().unwrap(), 
        z: coord_it.next().unwrap().as_str().parse::<i32>().unwrap()
    }
}

fn parse_scanner(input: Pair<Rule>) -> Scanner {
    let mut name = "unknown";
    let mut coords: HashSet<Coord> = HashSet::new();
    for rule in input.into_inner() {
        match rule.as_rule() {
            Rule::scanner_number => {
                name = rule.as_str()
            },
            Rule::coord_list => {
                for coord_rule in rule.into_inner() {
                    let coord = parse_coord(coord_rule);
                    coords.insert(coord);
                }
            }
            _ => panic!("Unexpected rule at this point: {:?}", rule)
        }    
    }
    Scanner{name: name.to_string(), coords: coords}
}

fn parse_rule(input: Pair<Rule>) -> Scanners {
    match input.as_rule() {
        Rule::scanners => {
            let mut scanners = Scanners{elems: Vec::new()};
            for scanner_rule in input.into_inner()
            {
                let scanner = parse_scanner(scanner_rule);
                scanners.elems.push(scanner);
            }
            return scanners;
        },
        _ => panic!("Unexpected rule at this point: {:?}", input)
    }
}

fn parse(input: String) -> Scanners{
    let pairs = ScannerParser::parse(Rule::scanners, &input)
        .expect("Parse failed")
        .next().unwrap();

    parse_rule(pairs)
}

fn match_scanner(objects: &HashSet<Object>, scanner: &Scanner) -> Option<(Coord, Scanner)>
{
    for transform in vec![
        ((1, 0, 0), (0, 1, 0), (0, 0, 1)),
        ((1, 0, 0), (0, 0, -1), (0, 1, 0)),
        ((1, 0, 0), (0, -1, 0), (0, 0, -1)),
        ((1, 0, 0), (0, 0, 1), (0, -1, 0)),
        ((0, -1, 0), (1, 0, 0), (0, 0, 1)),
        ((0, 0, 1), (1, 0, 0), (0, 1, 0)),
        ((0, 1, 0), (1, 0, 0), (0, 0, -1)),
        ((0, 0, -1), (1, 0, 0), (0, -1, 0)),
        ((-1, 0, 0), (0, -1, 0), (0, 0, 1)),
        ((-1, 0, 0), (0, 0, -1), (0, -1, 0)),
        ((-1, 0, 0), (0, 1, 0), (0, 0, -1)),
        ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
        ((0, 1, 0), (-1, 0, 0), (0, 0, 1)),
        ((0, 0, 1), (-1, 0, 0), (0, -1, 0)),
        ((0, -1, 0), (-1, 0, 0), (0, 0, -1)),
        ((0, 0, -1), (-1, 0, 0), (0, 1, 0)),
        ((0, 0, -1), (0, 1, 0), (1, 0, 0)),
        ((0, 1, 0), (0, 0, 1), (1, 0, 0)),
        ((0, 0, 1), (0, -1, 0), (1, 0, 0)),
        ((0, -1, 0), (0, 0, -1), (1, 0, 0)),
        ((0, 0, -1), (0, -1, 0), (-1, 0, 0)),
        ((0, -1, 0), (0, 0, 1), (-1, 0, 0)),
        ((0, 0, 1), (0, 1, 0), (-1, 0, 0)),
        ((0, 1, 0), (0, 0, -1), (-1, 0, 0)),
    ] {
        let mut offsets: HashMap<Coord,usize> = HashMap::new();
        for object in objects.iter() {
            if let Object::Beacon(object_coord) = object {
                for scanner_coord in scanner.coords.iter() {
                    let ((xtx, xty, xtz), (ytx, yty, ytz), (ztx, zty, ztz)) = transform;
                    let offset = Coord{
                        x: (scanner_coord.x * xtx + scanner_coord.y * xty + scanner_coord.z * xtz) - object_coord.x,
                        y: (scanner_coord.x * ytx + scanner_coord.y * yty + scanner_coord.z * ytz) - object_coord.y,
                        z: (scanner_coord.x * ztx + scanner_coord.y * zty + scanner_coord.z * ztz) - object_coord.z
                    };
                    *offsets.entry(offset).or_insert(0) += 1;
                }
            }
        }

        for (offset, count) in offsets.iter() {
            if *count >= 12 {
                println!("Found match with scanner {} at offset {} in orientation {:?}", scanner.name, offset, transform);
                let ((xtx, xty, xtz), (ytx, yty, ytz), (ztx, zty, ztz)) = transform;
                let transformed_scanner = Scanner{
                    name: scanner.name.to_string(),
                    coords: HashSet::from_iter(scanner.coords.iter()
                        .map(|c| {
                            Coord{
                                x: c.x * xtx + c.y * xty + c.z * xtz - offset.x,
                                y: c.x * ytx + c.y * yty + c.z * ytz - offset.y,
                                z: c.x * ztx + c.y * zty + c.z * ztz - offset.z
                            }    
                        }))
                };
                return Some((*offset, transformed_scanner))
            }
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Read failed");

    let mut scanners = parse(input);
    //println!("Scanners:\n{}", scanners);

    let mut objects: HashSet<Object> = HashSet::new();
    for coord in scanners.elems[0].coords.iter() {
        objects.insert(Object::Beacon(*coord));
    }
    objects.insert(Object::Scanner(scanners.elems[0].name.to_string(), Coord{x: 0, y: 0, z: 0}));
    scanners.elems.remove(0);
    
    while scanners.elems.len() > 0 {
        for i in 0..scanners.elems.len() {
            let scanner = &scanners.elems[i];
            let result = match_scanner(&objects, scanner);
            if let Some((offset, transformed_scanner)) = result {
                for coord in transformed_scanner.coords.iter() {
                    objects.insert(Object::Beacon(*coord));
                }
                objects.insert(Object::Scanner(transformed_scanner.to_string(), offset));
                scanners.elems.remove(i);
                break;
            }
        }
    }

    let mut beacon_coords: Vec<Coord> = objects.iter().map(|o| match o {
        Object::Beacon(c) => Some(c),
        Object::Scanner(_, _) => None
    }).filter(|c| c.is_some()).map(|c| *c.unwrap()).collect();
    beacon_coords.sort();
    for c in beacon_coords {
        println!("{}", c);
    }

    let beacon_count = objects.iter().filter(|o| match o {
        Object::Beacon(_) => true,
        Object::Scanner(_, _) => false
    }).count();

    println!("Number of beacons: {}", beacon_count);

    let mut max_mh = 0;
    let scanner_coords: Vec<Coord> = objects.iter().map(|o| match o {
        Object::Beacon(_) => None,
        Object::Scanner(_, c) => Some(c)
    }).filter(|c| c.is_some()).map(|c| *c.unwrap()).collect();
    for i in 0..scanner_coords.len() {
        for j in 0..scanner_coords.len() {
            let a = scanner_coords[i];
            let b = scanner_coords[j];
            let mh = (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs();
            if mh > max_mh {
                max_mh = mh;
            }
        }
    }

    println!("Max mahnattan distance: {}", max_mh);
}
