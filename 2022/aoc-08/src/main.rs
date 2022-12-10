use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn populate_visible<I: Iterator<Item = (usize, i32)>, F: FnMut(usize) -> ()>(
    trees: I,
    mut callback: F,
) -> i32 {
    trees.fold(-1, |largest_tree, (coord, height)| match height {
        _ if height > largest_tree => {
            callback(coord);
            height
        }
        _ => largest_tree,
    })
}

fn scenic_score(forest: &Vec<Vec<i32>>, x: usize, y: usize) -> usize {
    let w = forest[0].len();
    let h = forest.len();
    let height = forest[y][x];
    let sl = scenic_distance(height, forest, (0..x).rev().map(|xm| (xm, y)), x);
    let sr = scenic_distance(height, forest, (x + 1..w).map(|xm| (xm, y)), w - x - 1);
    let st = scenic_distance(height, forest, (0..y).rev().map(|ym| (x, ym)), y);
    let sb = scenic_distance(height, forest, (y + 1..h).map(|ym| (x, ym)), h - y - 1);
    return sl * sr * st * sb;
}

fn scenic_distance<I: Iterator<Item = (usize, usize)>>(
    height: i32,
    forest: &Vec<Vec<i32>>,
    coords: I,
    max_distance: usize,
) -> usize {
    match coords
        .enumerate()
        .skip_while(|(_, (x, y))| forest[*y][*x] < height)
        .map(|(c, _)| c)
        .nth(0)
    {
        Some(d) => d + 1,
        None => max_distance,
    }
}

fn main() {
    let forest: Vec<Vec<i32>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap() as i32).collect())
        .collect();
    let width = forest[0].len();

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    // left to right
    for (y, line) in forest.iter().enumerate() {
        populate_visible(line.iter().map(|&h| h).enumerate(), |x| {
            visible_trees.insert((x, y));
        });
    }

    // right to left
    for (y, line) in forest.iter().enumerate().rev() {
        populate_visible(line.iter().map(|&h| h).enumerate().rev(), |x| {
            visible_trees.insert((x, y));
        });
    }

    // top to bottom
    for x in 0..width {
        populate_visible(forest.iter().enumerate().map(|(y, l)| (y, l[x])), |y| {
            visible_trees.insert((x, y));
        });
    }

    // bottom to top
    for x in 0..width {
        populate_visible(
            forest.iter().enumerate().rev().map(|(y, l)| (y, l[x])),
            |y| {
                visible_trees.insert((x, y));
            },
        );
    }

    println!("Visible trees: {}", visible_trees.len());
    let max_scenic_score = (0..forest.len())
        .flat_map(|y| (0..forest[0].len()).map(move |x| (x, y)))
        .map(|(x, y)| scenic_score(&forest, x, y))
        .max()
        .unwrap();
    println!("Max scenic score: {}", max_scenic_score);
}
