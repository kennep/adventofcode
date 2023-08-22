use regex::{Regex, Captures};
use std::io::{stdin, BufRead};

#[derive(Debug,Copy,Clone)]
struct RobotCost {
    ore: usize,
    clay: usize,
    obsidian: usize
}

#[derive(Debug,Copy,Clone)]
struct Blueprint {
    id: usize,
    ore_robot: RobotCost,
    clay_robot: RobotCost,
    obsidian_robot: RobotCost,
    geode_robot: RobotCost
}

fn get_int(captures: &Captures, idx: usize) -> usize {
    captures.get(idx).unwrap().as_str().parse::<usize>().unwrap()
}

fn main() {
    let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let blueprints: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let captures = regex.captures(&l).unwrap();
            Blueprint{
                id: get_int(&captures, 1),
                ore_robot: RobotCost {
                    ore: get_int(&captures, 2),
                    clay: 0,
                    obsidian: 0
                },
                clay_robot: RobotCost {
                    ore: get_int(&captures, 3),
                    clay: 0,
                    obsidian: 0
                },
                obsidian_robot: RobotCost {
                    ore: get_int(&captures, 4),
                    clay: get_int(&captures, 5),
                    obsidian: 0
                },
                geode_robot: RobotCost {
                    ore: get_int(&captures, 6),
                    clay: 0,
                    obsidian: get_int(&captures, 7)
                }
            }
        })
        .collect();

    println!("{:?}", blueprints);
}
