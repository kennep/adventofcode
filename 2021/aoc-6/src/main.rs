use std::io::stdin;

fn num_fish(fish: &[usize]) -> usize
{
    fish.iter().sum()
}

fn main() {
    let mut inputstr = String::new();

    stdin().read_line(&mut inputstr).unwrap();

    let input: Vec<usize> = inputstr.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let mut all_fish: Vec<usize> = vec![0; 9];

    for lanternfish in input.iter() {
        all_fish[*lanternfish] += 1;
    }

    let days = 256;
    println!("Initial state: {:?}", num_fish(&all_fish));
    for day in 0..days {
        let fish_to_add = all_fish[0];

        for idx in 1..all_fish.len() {
            all_fish[idx - 1] = all_fish[idx];
        }
        all_fish[6] += fish_to_add;
        all_fish[8] = fish_to_add;
        println!("After {} days: {:?}", day + 1, num_fish(&all_fish));
    }

}
