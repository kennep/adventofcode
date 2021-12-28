use std::io::{stdin,Read};
use std::fmt;
use std::cmp::{min, max};
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct StepParser;

#[derive(Debug, PartialEq)]
enum Instruction {
    On,
    Off
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct CoordRange {
    min: i64,
    max: i64
}

impl CoordRange {
    fn intersects(&self, other: &CoordRange) -> bool {
        match self.intersection(other) {
            Some(_) => true,
            None => false
        }
    }

    fn intersection(&self, other: &CoordRange) -> Option<CoordRange> {
        let overlap_start = max(self.min, other.min);
        let overlap_end = min(self.max, other.max);

        if overlap_start <= overlap_end {
            Some(CoordRange{min: overlap_start, max: overlap_end})
        } else {
            None
        }
    }

    fn length(&self) -> i64 {
        // max is inclusive
        self.max - self.min + 1 
    }
}

#[derive(Debug)]
struct Step {
    instruction: Instruction,
    cuboid: Cuboid
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Cuboid {
    x: CoordRange,
    y: CoordRange,
    z: CoordRange
}

impl Cuboid {
    fn intersects(&self, other: &Cuboid) -> bool{
        match self.intersection(other) {
            Some(_) => true,
            None => false
        } 
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let x_intersection = self.x.intersection(&other.x);
        let y_intersection = self.y.intersection(&other.y);
        let z_intersection = self.z.intersection(&other.z);

        match (x_intersection, y_intersection, z_intersection) {
            (Some(x), Some(y), Some(z)) => 
                Some(Cuboid{x, y, z}),
            _ => None
        }
    }

    fn volume(&self) -> i64 {
        self.x.length() * self.y.length() * self.z.length()
    }

    fn diffsplit(&self, intersection: &Cuboid) -> Vec<Cuboid> {
        let mut result: Vec<Cuboid> = Vec::new();

        for xrange in [CoordRange{min: self.x.min, max: intersection.x.min - 1}, intersection.x, CoordRange{min: intersection.x.max + 1, max: self.x.max}] {
            for yrange in [CoordRange{min: self.y.min, max: intersection.y.min - 1}, intersection.y, CoordRange{min: intersection.y.max + 1, max: self.y.max}] {
                for zrange in [CoordRange{min: self.z.min, max: intersection.z.min - 1}, intersection.z, CoordRange{min: intersection.z.max + 1, max: self.z.max}] {
                    if xrange.length() <= 0 || yrange.length() <= 0 || zrange.length() <= 0 {
                        continue;
                    }
                    if xrange == intersection.x && yrange == intersection.y && zrange == intersection.z {
                        continue;
                    }
                    result.push(Cuboid{x: xrange, y: yrange, z: zrange});
                }
            }
        }

        result
    }

}

fn parse_step(input: Pair<Rule>) -> Step {
    let mut instruction: Option<Instruction> = None;
    let mut x: Option<CoordRange> = None;
    let mut y: Option<CoordRange> = None;
    let mut z: Option<CoordRange> = None;
    for rule in input.into_inner() {
        match rule.as_rule() {
            Rule::instruction => {
                match rule.as_str() {
                    "on" => instruction = Some(Instruction::On),
                    "off" => instruction = Some(Instruction::Off),
                    _ => panic!("Unexpected instruction: {:?}", rule)
                }
            },
            Rule::range => {
                let mut it = rule.into_inner();
                let axis = it.next().unwrap().as_str();
                let min: i64 = it.next().unwrap().as_str()
                    .parse().unwrap();
                let max: i64 = it.next().unwrap().as_str()
                    .parse().unwrap();
                match axis {
                    "x" => x = Some(CoordRange{min, max}),
                    "y" => y = Some(CoordRange{min, max}),
                    "z" => z = Some(CoordRange{min, max}),
                    _ => panic!("Unexpected axis: {:?}", axis)
                };
            }
            _ => panic!("Unexpected rule at this point: {:?}", rule)
        }    
    }
    Step {
        instruction: instruction.unwrap(),
        cuboid: Cuboid{
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),

        }
    }
}

fn parse_rule(input: Pair<Rule>) -> Vec<Step> {
    let mut result: Vec<Step> = Vec::new();
    for rule in input.into_inner() {
        match rule.as_rule() {
            Rule::step => {
                result.push(parse_step(rule));
            }
            _ => panic!("Unexpected rule at this point: {:?}", rule)
        }    

    }

    result
}

fn parse(input: String) -> Vec<Step> {
    let pairs = StepParser::parse(Rule::steps, &input)
        .expect("Parse failed")
        .next().unwrap();

    parse_rule(pairs)
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Read failed");

    let mut steps = parse(input);
    println!("{:?}", steps);

    let mut cubes = vec![vec![vec![false; 101]; 101]; 101];
    for step in steps.iter() {
        let cuboid = &step.cuboid;
        if cuboid.x.min < -50 || cuboid.x.min > 50 {
            continue;
        }
        for x in cuboid.x.min..=cuboid.x.max {
            for y in cuboid.y.min..=cuboid.y.max {
                for z in cuboid.z.min..=cuboid.z.max {
                    cubes[(x + 50) as usize][(y+50) as usize][(z+50) as usize] = match step.instruction {
                        Instruction::On => true,
                        Instruction::Off => false
                    }
                }
            }
        } 
    }

    let mut lit_count = 0;
    for x in 0..=100 {
        for y in 0..=100 {
            for z in 0..=100 {
                if cubes[x][y][z] {
                    lit_count += 1;
                }
            }
        }
    }

    println!("Lit cubes after initialization procedure: {}", lit_count);

    let mut cubes: Vec<Cuboid> = Vec::new();
    let first = steps.remove(0);
    cubes.push(first.cuboid);
    while steps.len() > 0 {
        println!("Cubes: {} Steps remaining: {}", cubes.len(), steps.len());
        let step = steps.remove(0);
        if step.instruction == Instruction::On {
            let mut add_ok = true;
            for i in 0..cubes.len() {
                let cube = cubes[i];
                if cube == step.cuboid {
                    println!("Already on");
                    add_ok = false;
                    break;
                }
                if let Some(intersection) = cube.intersection(&step.cuboid) {
                    println!("On: Cube {:?} intersects with {:?}: {:?}", cube, step.cuboid, intersection);
                    for splitcube in step.cuboid.diffsplit(&intersection) {
                        steps.insert(0, Step{
                            instruction: Instruction::On,
                            cuboid: splitcube
                        });
                    }
                    println!("On - intersection");
                    add_ok = false;
                    break;
                }
            }
            if add_ok {
                cubes.push(step.cuboid);
            }
        } else {
            for i in 0..cubes.len() {
                let cube = cubes[i];
                if cube == step.cuboid {
                    cubes.remove(i);
                    println!("Removing cube {} due to off instruction", i);
                    break;
                }
                if let Some(intersection) = cube.intersection(&step.cuboid) {
                    cubes.remove(i);
                    for splitcube in cube.diffsplit(&intersection) {
                        cubes.push(splitcube);
                    }
                    for splitcube in step.cuboid.diffsplit(&intersection) {
                        steps.insert(0, Step{
                            instruction: Instruction::Off,
                            cuboid: splitcube
                        });
                    }
                    println!("Off - intersection");
                    break;
                }
            }
        }
    }

    let lit_count: i64 = cubes.iter().map(|c| c.volume()).sum();
    println!("Lit cubes after reboot: {}", lit_count);
}
