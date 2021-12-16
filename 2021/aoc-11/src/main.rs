use std::io::{stdin, BufRead};

struct Octopus {
    energy: u32,
    flashed: bool
}

fn flash(input: &mut Vec<Vec<Octopus>>, x: usize, y: usize)
{
    input[y][x].flashed = true;
    if y > 0 {
        if x > 0 {
            input[y-1][x-1].energy += 1;
        }
        input[y-1][x].energy += 1;
        if x < input[0].len() - 1 {
            input[y-1][x+1].energy += 1;
        }
    }
    if x > 0 {
        input[y][x-1].energy += 1;
    }
    if x < input[0].len() - 1 {
        input[y][x+1].energy += 1;
    }
    if y < input.len() - 1 {
        if x > 0 {
            input[y+1][x-1].energy += 1;
        }
        input[y+1][x].energy += 1;
        if x < input[0].len() - 1 {
            input[y+1][x+1].energy += 1;
        }
    }
}

fn main() {
    let mut input: Vec<Vec<Octopus>> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(|c| Octopus{ energy: c.to_digit(10).unwrap(), flashed: false}).collect())
        .collect();

    let width = input[0].len();
    let height = input.len();
    let steps = 100;
    let mut num_flashes = 0;
    for step in 0..10000 {
        for y in 0..height {
            for x in 0..width {
                input[y][x].energy += 1;
            }
        }
        loop {
            let mut flashed: bool = false;
            for y in 0..height {
                for x in 0..width {
                    if input[y][x].energy > 9 && !input[y][x].flashed {
                        flash(&mut input, x, y);
                        flashed = true;
                        num_flashes += 1;
                    }
                }
            }
            if !flashed {
                break;
            }    
        }
        for y in 0..height {
            for x in 0..width {
                if input[y][x].flashed {
                    input[y][x].energy = 0;
                    input[y][x].flashed = false;
                }
            }
        }
        if step + 1 == steps {
            println!("Number of flashes after {} steps: {}", step + 1, num_flashes);
        }
        let mut allzeros = true;
        for y in 0..height {
            for x in 0..width {
                if input[y][x].energy > 0 {
                    allzeros = false;
                }
            }
        }
        if allzeros {
            println!("All octopuses flashed simultaneously on step: {}", step + 1);
            break;
        }

    }

}
