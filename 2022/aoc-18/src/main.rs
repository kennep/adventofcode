use std::collections::{HashSet, VecDeque};
use std::io::{stdin, BufRead};

fn neighbors((x, y, z): (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn is_interior(
    blocks: &HashSet<(i32, i32, i32)>,
    interior_blocks: &mut HashSet<(i32, i32, i32)>,
    exterior_blocks: &mut HashSet<(i32, i32, i32)>,
    start: (i32, i32, i32),
) -> bool {
    if interior_blocks.contains(&start) {
        return true;
    }
    if exterior_blocks.contains(&start) {
        return false;
    }

    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    q.push_back(start);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for nb in neighbors(v) {
            if blocks.contains(&nb) {
                continue;
            }
            if !visited.contains(&nb) {
                visited.insert(nb);
                q.push_back(nb);
            }
        }
        if visited.len() > 2000 {
            exterior_blocks.extend(&visited);
            return false;
        }
    }
    interior_blocks.extend(&visited);
    true
}

fn main() {
    let blocks: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|s| (s[0], s[1], s[2]))
        .collect();

    let blockset: HashSet<_> = HashSet::from_iter(blocks.into_iter());

    let exposed_sides = blockset
        .iter()
        .map(|b| {
            let nb = neighbors(*b);
            nb.iter()
                .map(|c| match blockset.contains(c) {
                    true => 0,
                    false => 1,
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Number of exposed sides: {}", exposed_sides);

    let mut known_exterior_blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut known_interior_blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    let exposed_sides = blockset
        .iter()
        .map(|b| {
            let nb = neighbors(*b);
            nb.iter()
                .map(|c| match blockset.contains(c) {
                    true => 0,
                    false => match is_interior(
                        &blockset,
                        &mut known_interior_blocks,
                        &mut known_exterior_blocks,
                        *c,
                    ) {
                        true => 0,
                        false => 1,
                    },
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Number of exposed exterior sides: {}", exposed_sides);
}
