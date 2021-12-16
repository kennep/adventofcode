use std::io::{stdin,BufRead};
use std::collections::HashMap;

#[derive(PartialEq)]
enum CaveSize {
    Big,
    Small
}

struct Cave {
    name: String,
    exits: Vec<String>
}

fn cavesize(name: &str) -> CaveSize
{
    if name.chars().all(|c| c.is_ascii_uppercase()) {
        return CaveSize::Big;
    }
    CaveSize::Small
}

fn num_routes(caves: &HashMap<String, Cave>, start: &str, visited: Vec<String>) -> u32
{
    let cave = caves.get(start).unwrap();
    if cave.name == "end" {
        //println!("Found route: {:?} -> end", visited);
        return 1;
    }

    let possible_exits = cave.exits.iter().filter(
        |exit| match cavesize(exit) {
            CaveSize::Big => true,
            CaveSize::Small => !visited.contains(exit)
        }
    );

    possible_exits.map(|exit| {
        let mut visited_for_exit = visited.clone();
        visited_for_exit.push(exit.clone());
        num_routes(caves, exit, visited_for_exit)
    }).sum()
}

fn num_routes2(caves: &HashMap<String, Cave>, start: &str, visited: Vec<&str>) -> u32
{
    let cave = caves.get(start).unwrap();
    if cave.name == "end" {
        //println!("Found route: {:?} -> end", visited);
        return 1;
    }

    let possible_exits = cave.exits.iter().filter(
        |exit| match cavesize(exit) {
            CaveSize::Big => true,
            CaveSize::Small => *exit != "start" && 
                (!visited.iter().any(|e| e == exit) ||
                !visited.iter().any(|c| cavesize(c) == CaveSize::Small && visited.iter().filter(|cn| *cn == c).count() >= 2))
        }
    );

    possible_exits.map(|exit| {
        let mut visited_for_exit = visited.clone();
        visited_for_exit.push(exit);
        num_routes2(caves, exit, visited_for_exit)
    }).sum()
}

fn main() {
    let input: Vec<(String, String)> = 
        stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.split('-').map(String::from).collect::<Vec<String>>())
        .map(|s| (s[0].clone(), s[1].clone()))
        .collect();

    let mut cave_map: HashMap<String, Cave> = HashMap::new();

    for (source, dest) in input.iter() {
        let source_cave = cave_map.entry(source.clone()).or_insert(Cave {
            name: source.clone(),
            exits: Vec::new()
        });
        source_cave.exits.push(dest.clone());
        let dest_cave = cave_map.entry(dest.clone()).or_insert(Cave {
            name: dest.clone(),
            exits: Vec::new()
        });
        dest_cave.exits.push(source.clone());
    }

    let routes = num_routes(&cave_map, "start", vec![String::from("start")]);
    println!("A: Number of possible routes: {}", routes);

    let routes = num_routes2(&cave_map, "start", vec!["start"]);
    println!("B: Number of possible routes: {}", routes);
}
