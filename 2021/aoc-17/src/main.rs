use std::collections::HashMap;
use std::env;

#[derive(Copy, Clone)]
struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // target area: x=20..30, y=-10..-5
    // target area: x=60..94, y=-171..-136
    let datasets = HashMap::from([
        ("example".to_string(), Target{min_x: 20, max_x: 30, min_y: -10, max_y: -5}),
        ("input".to_string(), Target{min_x: 60, max_x: 94, min_y: -171, max_y: -136})
    ]);

    let dataset_name = args[1].to_string();
    let target = datasets[&dataset_name];

    let mut best_y = 0;
    let mut best_xiv = 0;
    let mut best_yiv = 0;
    let mut distict_velocities = 0;

    for xiv in 0..1000 {
        for yiv in -1000..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut xv = xiv;
            let mut yv = yiv;
            let mut highest_y = 0;
            for _ in 0..1000 {
                x += xv;
                y += yv;
                if xv > 0 {
                    xv-= 1;
                }
                yv -= 1;
                if y > highest_y {
                    highest_y = y;
                }
                if x >= target.min_x && x <= target.max_x &&
                    y >= target.min_y && y <= target.max_y {
                    distict_velocities += 1;
                    if highest_y > best_y {
                        best_y = highest_y;
                        best_xiv = xiv;
                        best_yiv = yiv;
                    }
                    break;
                }
                if x > target.max_x || y < target.min_y {
                    break;
                }
            }
        }
    }

    println!("Dataset: {}", dataset_name);
    println!("Best solution: highest y={}, xv={}, yv={}", best_y, best_xiv, best_yiv);
    println!("Distinct velocities: {}", distict_velocities);
}
