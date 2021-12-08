use std::io::{stdin, BufRead};

fn sumint(n: i32) -> i32 {
    let mut s = n;
    let mut t = 0;
    while s > 0 {
        t += s;
        s -= 1;
    }
    t
}

fn main() {
    let mut input: Vec<i32> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .flat_map(|l| l.split(',').map(|n| n.parse::<i32>().unwrap_or_else(|_| panic!("Expected number, not {}", n))).collect::<Vec<i32>>())
        .collect();

    println!("Input: {:?}", input);

    input.sort();

    println!("Sorted input: {:?}", input);

    let min = input[0];
    let max = input[input.len() - 1];

    let mut min_fuel: Option<i32> = None;
    let mut min_pos: Option<i32> = None;
    for test_pos in min..=max {
        let fuel = input.iter().map(|pos| (test_pos - pos).abs()).sum();
        match min_fuel {
            None => {
                min_fuel = Some(fuel);
                min_pos = Some(test_pos);
            },
            Some(f) if fuel < f => {
                min_fuel = Some(fuel);
                min_pos = Some(test_pos);
                //println!("Found lower fuel {} at pos {}, avg: {}", fuel, test_pos, (fuel as f64) / input.len() as f64);
            },
            _ => {
                //println!("Higher fuel {} at pos {}, avg: {}", fuel, test_pos, (fuel as f64) / input.len() as f64);
            }
        }
    }
    println!("A: min fuel {} pos: {}", min_fuel.unwrap(), min_pos.unwrap());

    let mut min_fuel: Option<i32> = None;
    let mut min_pos: Option<i32> = None;
    for test_pos in min..=max {
        let fuel = input.iter().map(|pos| sumint((test_pos - pos).abs())).sum();
        match min_fuel {
            None => {
                min_fuel = Some(fuel);
                min_pos = Some(test_pos);
            },
            Some(f) if fuel < f => {
                min_fuel = Some(fuel);
                min_pos = Some(test_pos);
                //println!("Found lower fuel {} at pos {}, avg: {}", fuel, test_pos, (fuel as f64) / input.len() as f64);
            },
            _ => {
                //println!("Higher fuel {} at pos {}, avg: {}", fuel, test_pos, (fuel as f64) / input.len() as f64);
            }
        }
    }
    println!("B: min fuel {} pos: {}", min_fuel.unwrap(), min_pos.unwrap());

}
