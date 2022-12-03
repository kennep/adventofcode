use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn priority(c: char) -> u32
{
    match c
    {
        'a'..='z' => 1 + (c as u32) - ('a' as u32),
        'A'..='Z' => 27 + (c as u32) - ('A' as u32),
        _ => 0
    }
}

fn main() {
    let rucksacks: Vec<_>  = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();

    let rucksacks_with_compartments: Vec<_> = rucksacks.iter()
        .map(|l| (l[0..l.len() / 2].to_owned(), l[l.len() / 2 ..].to_owned()))
        .collect()
        ;

    println!("A-----------------------------------------------");
    println!("Rucksacks: {:?}", rucksacks_with_compartments);

    let common_elements: Vec<_> = rucksacks_with_compartments.iter()
        .map(|(f, s)| f.chars().filter(|c| s.contains(*c)).collect::<HashSet<_>>())
        .collect();

    println!("Common elements: {:?}", common_elements);

    let priorities: Vec<_> = common_elements.iter().flatten().map(|c| priority(*c)).collect();

    println!("Priorities: {:?}", priorities);

    println!("Sum of priorities: {}", priorities.iter().sum::<u32>());


    println!("B-----------------------------------------------");

    let groups: Vec<_> = rucksacks.chunks(3).collect();

    println!("Groups: {:?}", groups);

    let group_common_elements: Vec<_> = groups.iter()
        .map(|g| g[0].chars().filter(|c| g[1].contains(*c)).filter(|c| g[2].contains(*c)).collect::<HashSet<_>>())
        .collect();

    println!("Group common elements: {:?}", group_common_elements);

    let group_priorities: Vec<_> = group_common_elements.iter().flatten().map(|c| priority(*c)).collect();

    println!("Priorities: {:?}", group_priorities);

    println!("Sum of priorities: {}", group_priorities.iter().sum::<u32>());

}
