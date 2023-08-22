use std::{io::{stdin, BufRead}, cmp::{max, min}, collections::{VecDeque, HashSet}};
use std::collections::HashMap;
use itertools::{self, Itertools};
use regex::Regex;

#[derive(Clone,Debug,Eq,PartialEq)]
struct ConnectedRoom {
    id: String,
    cost: usize
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Valve {
    id: String,
    flow_rate: usize,
    exits: Vec<ConnectedRoom>
}

enum Action<'a> {
    Move(&'a str),
    Open
}

impl Valve {
    fn get_actions<'a>(&'a self, came_from: &'a str, open_valves: &Vec<&'a str>) -> Vec<Action<'a>>{
        let mut actions: Vec<_> = self.exits.iter()
            .filter(|e| e.id != came_from)
            .map(|e| Action::Move(e.id.as_str()))
            .collect();
            
        if self.flow_rate > 0 && !open_valves.contains(&self.id.as_str()) {
            actions.push(Action::Open);
        }
            
        actions
    }

    fn step<'a>(&'a self, valves: &'a HashMap<String, Valve>, time_left: usize, came_from: &'a str, open_valves: &Vec<&'a str>, acc_release: usize) -> usize {
        if time_left == 0 {
            return acc_release;
        }
        self.get_actions(came_from, open_valves).iter().map(|action| {
            match action {
                Action::Move(e) => valves.get(*e).unwrap().step(valves, time_left - 1, &self.id, open_valves, acc_release),
                Action::Open => {
                    let mut new_open_valves = open_valves.clone();
                    new_open_valves.push(self.id.as_str());
                    self.step(valves, time_left - 1, "", &new_open_valves, acc_release + (time_left  - 1) * self.flow_rate)
                }
            }
        }).max().unwrap_or(acc_release)
    } 
}

/*
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
 */

fn build_exits(room: & str, valves: & HashMap<String, Valve>) -> Valve
{
    let mut q: VecDeque<(&str, usize)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert(room);

    let mut exits: Vec<ConnectedRoom> = Vec::new();

    q.push_back((room, 0));
    while !q.is_empty() {
        let (node, cost) = q.pop_front().unwrap();
        let valve = &valves[node];
        for edge in &valve.exits {
            if !visited.contains(&*edge.id) {
                let edge_valve = &valves[&edge.id];
                visited.insert(&edge.id);
                if edge_valve.flow_rate > 0 {
                    exits.push(ConnectedRoom { id: edge.id.to_owned(), cost: cost + edge.cost + 1})
                }
                q.push_back((&edge.id, cost + edge.cost));
            } 
        }
    }

    Valve { id: room.to_owned(), flow_rate: valves[room].flow_rate, exits: exits }
}

fn main() {
    let lines = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty());

    let regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

    let valves: Vec<_> = lines
        .map(|l| {
            if let Some(caps) = regex.captures(&l) {
                let tunnels: Vec<_> = caps.get(3).unwrap().as_str().split(", ")
                    .map(|s| s.to_owned()).collect();
                return Valve {
                    id: caps.get(1).unwrap().as_str().to_owned(),
                    flow_rate: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    exits: tunnels.iter().map(|t| ConnectedRoom{id: t.to_owned(), cost: 1}).collect()
                };
            }
            panic!("Unexpected input: {}", l);
        })
        .collect();

    let valves: HashMap<String, Valve> = HashMap::from_iter(valves.into_iter().map(|v| (v.id.to_owned(), v)));

    println!("{:?}", valves);

    let aa_connected = build_exits("AA", &valves);
    let mut valves_connected: HashMap<String, Valve> = HashMap::new();
    valves_connected.insert("AA".to_owned(), aa_connected);
    for v in valves.values() {
        let node_connected = build_exits(&v.id, &valves);
        valves_connected.insert(v.id.to_owned(), node_connected);
    }

    let mut time_left = 30;
    let mut current_node: &Valve = &valves_connected["AA"];
    let mut max_accum_release = 0;
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert(&current_node.id);
    while time_left > 0 {
        println!("In room {}", current_node.id);
        let choices = current_node.exits.iter()
            .filter(|e| !visited.contains(&*e.id))
            .filter(|e| e.cost < time_left + 1)
            .map(|e| (&e.id, e.cost, valves_connected[&e.id].flow_rate * (time_left - e.cost)))
            .collect::<Vec<_>>();
        println!("Choices: {:?}", choices);

        let choice = choices.into_iter()
            .max_by(|a, b| ((a.2/a.1).cmp(&(b.2/b.1))));
        if let Some((target, cost, value)) = choice {
            println!("Moving to room {}, cost = {}, value = {}", target, cost, value);
            current_node = &valves_connected[target];
            time_left -= cost;
            max_accum_release += value;
            visited.insert(&current_node.id);
        } else {
            break;
        }
    }
    
    //let max_accum_release = valves.get("AA").unwrap().step(&valves, 30, "", &Vec::new(), 0);

    println!("Max accumulated pressure release: {}", max_accum_release);


}
