use std::{io::{stdin, BufRead}, str::FromStr, num::ParseIntError};
use std::cmp::{max,min};

#[derive(Debug,Copy,Clone, PartialEq, Eq)]
struct AssignmentRange
{
    start: u32,
    end: u32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAssignmentRangeError {
    range: String
}

impl From<ParseIntError> for ParseAssignmentRangeError
{
    fn from(err: ParseIntError) -> Self {
        ParseAssignmentRangeError{ range: err.to_string() }   
    }
}

impl AssignmentRange
{
    fn contains(&self, other: &AssignmentRange) -> bool
    {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps_with(&self, other: &AssignmentRange) -> bool
    {
        max(self.start, other.start) <= min(self.end, other.end)
    }
}

impl FromStr for AssignmentRange
{
    type Err = ParseAssignmentRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges:Vec<_> = s.split("-").collect();
        if ranges.len() != 2 {
            return Err(ParseAssignmentRangeError { range: s.to_owned() })
        }

        Ok(AssignmentRange { 
            start: ranges[0].parse::<u32>()?,
            end: ranges[1].parse::<u32>()?
        })
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();

    let groups: Vec<_> = lines.iter()
        .map(|s| s.split(",").collect::<Vec<_>>())
        .map(|g| (g[0].parse::<AssignmentRange>().unwrap(), g[1].parse::<AssignmentRange>().unwrap()))
        .collect();

    println!("Groups: {:?}", groups);

    let contained_ranges: Vec<_> = groups.iter().filter(|(e1, e2)| e1.contains(e2) || e2.contains(e1)).collect();

    println!("Contained ranges: {:?}\nNumber of ranges:{}", contained_ranges, contained_ranges.len());

    let overlapped_ranges: Vec<_> = groups.iter().filter(|(e1, e2)| e1.overlaps_with(e2)).collect();

    println!("Overlapped ranges: {:?}\nNumber of ranges:{}", overlapped_ranges, overlapped_ranges.len());

}
