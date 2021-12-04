use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<i32> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| l.parse().expect("failed to parse number"))
        .collect();

    let (increase_count, _) = input.iter().map(|i| *i)
        .fold((0, 10000), |(count, last_value), next_value| 
           match next_value {
               v if v > last_value => (count + 1, next_value),
               _ => (count, next_value)
           });
    println!("A: {}", increase_count);

    let (increase_count, _, _, _) = input.iter().map(|i| *i)
        .fold((0, 10000, -1, -1), |(count, last_sum, prev_1, prev_2), next_value| {
            match prev_2 {
                -1 => (count, last_sum, next_value, prev_1),
                _ => {
                    let current_sum = next_value + prev_1 + prev_2;
                    (match current_sum {
                        v if v > last_sum => count + 1,
                        _ => count
                    }, current_sum, next_value, prev_1)
                }
            }
        });
    println!("B: {}", increase_count);
}
