use std::io::{stdin,BufRead};
use std::cmp::{max,min};

fn get(input: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    input[y as usize][x as usize]
}

fn neighbors(input: &Vec<Vec<i32>>, x: i32, y: i32) -> Vec<i32>
{
    let mut n: Vec<i32> = Vec::new();
    if x > 0 {
        n.push(get(input, x - 1, y))
    }
    if y > 0 {
        n.push(get(input, x, y - 1))
    }
    if x < (input[0].len() - 1) as i32 {
        n.push(get(input, x + 1, y))
    }
    if y < (input.len() - 1) as i32 {
        n.push(get(input, x, y + 1))
    }
    n
}

#[derive(Debug,Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct BasinSpan {
    size: i32,
    y: i32,
    begin_x: i32,
    end_x: i32
}

impl BasinSpan {
    fn is_adjacent(&self, other: &BasinSpan) -> bool
    {
        if self.y != other.y - 1 && self.y != other.y + 1 {
            return false;
        }

        let max_s = max(self.begin_x, other.begin_x);
        let min_e = min(self.end_x, other.end_x);
        return min_e >= max_s;
    }
}

#[derive(Debug,Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Basin {
    size: i32,
    spans: Vec<BasinSpan>
}

impl Basin {
    fn new(span: &BasinSpan) -> Basin {
        Basin { size: span.size, spans: vec![span.clone()]}
    }

    fn is_adjacent_to_span(&self, span: &BasinSpan) -> bool {
        self.spans.iter().any(|s| s.is_adjacent(span))
    }

    fn is_adjacent_to_basin(&self, basin: &Basin) -> bool {
        self.spans.iter().any(|s| basin.is_adjacent_to_span(s))
    }

    fn add_span(&mut self, span: &BasinSpan) 
    {
        self.size += span.size;
        self.spans.push(span.clone())
    }

}

fn basin_spans(input: &Vec<i32>, y: i32) -> Vec<BasinSpan>
{
    let mut spans: Vec<BasinSpan> = Vec::new();
    let mut cur_basin: Option<usize> = None;
    for (x, v) in input.iter().enumerate() {
        match (*v, cur_basin) {
            (v, None) if v < 9 => {
                spans.push(BasinSpan{size: 1, y: y, begin_x: x as i32, end_x: x as i32});
                cur_basin = Some(spans.len() - 1);
            },
            (v, Some(b)) if v < 9 => {
                spans[b].size += 1;
                spans[b].end_x = x as i32;
            }
            (9, Some(_)) => {
                cur_basin = None;
            },
            _ => {}
        }
    }
    //println!("Spans for row {}: {:?}", y, spans);
    spans
}

fn basins(input: &Vec<Vec<i32>>) -> Vec<Basin>
{
    let mut basins: Vec<Basin> = Vec::new();
    for y in 0..input.len() {
        let basin_spans = basin_spans(&input[y], y as i32);
        for span in basin_spans.iter() {
            let (adjacent, not_adjacent): (Vec<&Basin>, Vec<&Basin>) = 
                basins.iter().partition(|b| b.is_adjacent_to_span(span));
            let mut combined_basin = Basin::new(span);
            for basin in adjacent.iter() {
                for bs in basin.spans.iter() {
                    combined_basin.add_span(bs);
                }
            }
            basins = not_adjacent.into_iter().map(|b| b.clone()).collect();
            basins.push(combined_basin);
        }
    }
    basins
}

fn main() {
    let input: Vec<Vec<i32>> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let width = input[0].len();
    let ir = &input;
    let risk: i32 = (0..input.len())
        .flat_map(|y| {
            (0..width).map(move |x| {
                let val = get(ir, x as i32, y as i32);
                let neigh = neighbors(ir, x as i32, y as i32);
                (val, neigh) 
            })
        })
        .filter(|(val, neigh) | {
            neigh.iter().filter(|v| *v > val).count() == neigh.len()
        })
        .map(|(val, _)| val + 1)
        .sum();
    println!("Risk score: {}", risk);

    let lowpoints: Vec<_> = (0..input.len())
    .flat_map(|y| {
        (0..width).map(move |x| {
            let val = get(ir, x as i32, y as i32);
            let neigh = neighbors(ir, x as i32, y as i32);
            (x as i32, y as i32, val, neigh) 
        })
    })
    .filter(|(_x, _y, val, neigh) | {
        neigh.iter().filter(|v| *v > val).count() == neigh.len()
    })
    .map(|(x, y, val, _)| (x, y, val))
    .collect();

    println!("Low points: {:?}", lowpoints);

    let mut basins = basins(&input);
    basins.sort();
    for b in basins.iter() {
        println!("Basin: {:?}", b);
    }

    let l = basins.len();
    println!("Product largest three: {:?}", basins[l-1].size * basins[l-2].size * basins[l-3].size);

}
