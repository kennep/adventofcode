use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn find_marker(input: &str, marker_size: usize) -> Option<usize>
{
    for (pos, window) in input.chars().collect::<Vec<char>>().windows(marker_size).enumerate() {
        let charset: HashSet<char>= window.iter().map(|c| *c).collect();
        if charset.len() == marker_size 
        {
            return Some(pos + marker_size);
        }
    }
    None
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect()
        ;
    
    for line in lines.iter() {
        let pos = find_marker(&line, 4);
        match pos {
            Some(p) => println!("Packet marker found after {} chars", p),
            None => println!("No packet marker found")
        };
    }

    for line in lines.iter() {
        let pos = find_marker(&line, 14);
        match pos {
            Some(p) => println!("Message marker found after {} chars", p),
            None => println!("No message marker found")
        };
    }

}
