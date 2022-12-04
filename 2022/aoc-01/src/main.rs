use std::io::{stdin, BufRead};

fn main() {
    let lines: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();

    let elves_calories = lines
        .split(|l| l.is_empty())
        .map(|c| {
            c.iter()
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum_calories: Vec<i32> = elves_calories
        .iter()
        .map(|c| c.iter().sum())
        .collect::<Vec<_>>();
    let max_calories = sum_calories.iter().max().unwrap();

    println!("Elves: {:?}", elves_calories);
    println!("Calories: {:?}", sum_calories);
    println!("Max calories: {}", max_calories);

    sum_calories.sort();

    let top_three = sum_calories.into_iter().rev().take(3).collect::<Vec<_>>();

    println!("Top 3 calories: {:?}", top_three);
    println!("Sum of top 3 calories: {}", top_three.iter().sum::<i32>());
}
