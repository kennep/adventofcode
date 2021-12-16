use std::io::{stdin, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::max;

#[derive(Copy, Clone)]
enum FoldInstruction {
    Horizontal(u32),
    Vertical(u32)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32
}

fn fold(sheet: &HashSet<Point>, fold_instruction: FoldInstruction) -> HashSet<Point>
{
    HashSet::from_iter(
        sheet.iter().map(|point| {
            match (point, fold_instruction) {
                (Point{x, y}, FoldInstruction::Horizontal(c)) if *y > c => 
                    Point{x: *x, y: 2 * c - *y},
                (Point{x, y}, FoldInstruction::Vertical(c)) if *x > c => 
                    Point{x: 2 * c - *x, y: *y},
                _ => *point
            }
        })
    )
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .collect();
    
    let mut coords_and_fold_instructions = lines[..].split(|l| l.is_empty());

    let sheet: HashSet<Point> = HashSet::from_iter(coords_and_fold_instructions.next().unwrap().iter()
        .map(|l| l.split(','))
        .map(|mut i| (i.next().unwrap().parse().unwrap(), i.next().unwrap().parse().unwrap()))
        .map(|(x, y)| Point{x, y}));

    let fold_instructions: Vec<FoldInstruction> = coords_and_fold_instructions.next().unwrap().iter()
        .map(|l| l.split('='))
        .map(|mut i| {
            let instruction = i.next().unwrap();
            let coord: u32 = i.next().unwrap().parse().unwrap();
            match instruction {
                "fold along y" => FoldInstruction::Horizontal(coord),
                "fold along x" => FoldInstruction::Vertical(coord),
                f => panic!("Unknown fold instruction {}", f)
            }
        })
        .collect();

    let sheet_after_1_fold = fold(&sheet, fold_instructions[0]);

    println!("Number of points after 1 fold: {}", sheet_after_1_fold.len());

    let folded_sheet = fold_instructions.iter()
        .fold(sheet, |sheet, instruction| fold(&sheet, *instruction));

    let max_x = folded_sheet.iter().fold(0, |m, p| max(m, p.x));
    let max_y = folded_sheet.iter().fold(0, |m, p| max(m, p.y));
    for y in 0..=max_y {
        for x in 0..=max_x {
            if folded_sheet.contains(&Point{x, y}) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
    
