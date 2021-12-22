use std::io::{stdin,BufRead};


fn main() {
    let input: Vec<i32> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| l.parse().unwrap_or_else(|_| panic!("Expected number, not {}", l)))
        .collect();

    let fuel_requirement: i32 = input
        .iter()
        .map(|f| f / 3 - 2)
        .sum();

    println!("A: fuel requirement: {}", fuel_requirement);

    let fuel_requirement: i32 = input
    .iter()
    .map(|module| {
        let mut total_fuel_mass:i32 = 0;
        let mut remaining_mass = *module;
        loop {
            let fuel_needed = remaining_mass / 3 - 2;
            if fuel_needed <= 0 {
                break;
            }
            total_fuel_mass += fuel_needed;
            remaining_mass = fuel_needed;
        }
        total_fuel_mass
     })
    .sum();

    println!("B: fuel requirement: {}", fuel_requirement);

}
