use std::io::{stdin,Read};
use std::fmt;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct StepParser;

#[derive(Debug)]
enum Instruction {
    On,
    Off
}

#[derive(Debug,Copy,Clone)]
struct CoordRange {
    min: i64,
    max: i64
}

#[derive(Debug)]
struct Step {
    instruction: Instruction,
    cuboid: Cuboid
}

#[derive(Debug,Copy,Clone)]
struct Cuboid {
    x: CoordRange,
    y: CoordRange,
    z: CoordRange
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

    let steps = parse(input);
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
}
