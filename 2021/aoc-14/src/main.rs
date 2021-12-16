use std::io::{stdin,BufRead};
use std::collections::HashMap;
use std::iter::FromIterator;

struct Rule {
    pattern: (char, char),
    insert: char
}

fn eval_polymer(polymer: &[char],  rules: &HashMap<(char, char), Rule>) -> Vec<char>
{
    let mut output: Vec<char> = Vec::new();
    let _ = polymer.iter().fold(
        '$', |last_char, next_char| {
            let maybe_rule = rules.get(&(last_char, *next_char));
            if let Some(rule) = maybe_rule {
                output.push(rule.insert);
            }
            output.push(*next_char);
            *next_char
        }
    );
    output
}

fn eval_polymer_2(polymer: &HashMap<(char, char), usize>, rules: &HashMap<(char, char), Rule>) -> HashMap<(char ,char), usize>
{
    let mut output: HashMap<(char ,char), usize> = HashMap::new();
    for (pair, count) in polymer.iter() {
        let maybe_rule = rules.get(pair);
        if let Some(rule) = maybe_rule {
            let pair1 = (rule.pattern.0, rule.insert);
            let pair2 = (rule.insert, rule.pattern.1);
            *output.entry(pair1).or_insert(0) += count;
            *output.entry(pair2).or_insert(0) += count;
        }
    }
    output
}

fn count_elements(polymer: &[char]) -> HashMap<char, usize>
{
    let mut result: HashMap<char, usize> = HashMap::new();

    for c in polymer.iter() {
        *(result.entry(*c).or_insert(0)) += 1;
    }

    result
}

fn count_elements_2(polymer: &HashMap<(char, char), usize>, first_char: char, last_char: char) -> HashMap<char, usize>
{
    let mut result: HashMap<char, usize> = HashMap::new();
    for ((first, last), count) in polymer.iter() {
        *(result.entry(*first).or_insert(0)) += count;
        *(result.entry(*last).or_insert(0)) += count;
    }

    for (k, v) in result.iter_mut() {
        let mut adj = 0;
        if *k == first_char{ adj += 1 }
        if *k == last_char { adj += 1 }
        *v = (*v + adj) / 2;
    }
    result
}

fn to_pairs(polymer: &[char]) -> HashMap<(char, char), usize> {
    let mut output: HashMap<(char ,char), usize> = HashMap::new();
    let _ = polymer.iter().fold(
        '$', |last_char, next_char| {
            if last_char != '$' {
                *output.entry((last_char, *next_char)).or_insert(0) += 1;
            }
            *next_char
        }
    );

    output
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
    .map(|l| l.unwrap())
    .collect();

    let template_and_polymer_rules: Vec<&[String]> = lines[..].split(|l| l.is_empty()).collect();

    let mut polymer: Vec<char> = template_and_polymer_rules[0][0].chars().collect();
    let first_char = polymer[0];
    let last_char = polymer[polymer.len() - 1];

    let mut polymer_pairs = to_pairs(&polymer);

    let rules: Vec<Rule> = template_and_polymer_rules[1].iter()
        .map(|r| r.split(" -> ").collect::<Vec<&str>>())
        .map(|r| {
            let mut rule_chars = r[0].chars();
            Rule{
                pattern: (rule_chars.next().unwrap(), rule_chars.next().unwrap()),
                insert: r[1].chars().next().unwrap()
            }
        })
        .collect();

    let rules: HashMap<(char, char), Rule> = HashMap::from_iter(
        rules.into_iter().map(|r| (r.pattern, r))
    );

    for _ in 0..10 {
        polymer = eval_polymer(&polymer, &rules);
        //println!("Step: {} Polymer length: {}", step, polymer.len());
    }

    let element_counts = count_elements(&polymer);

    let max_element = element_counts.iter()
        .fold(('$', 0), |(mk, mv), (k, v)| {
            if *v > mv {
                (*k, *v)
            } else {
                (mk, mv)
            }
        });

    let min_element = element_counts.iter()
        .fold(('$', usize::MAX), |(mk, mv), (k, v)| {
            if *v < mv {
                (*k, *v)
            } else {
                (mk, mv)
            }
        });

    println!("After 10 steps:");
    println!("Most common element: {:?}", max_element);
    println!("Least common element: {:?}", min_element);

    println!("Result: {}", max_element.1 - min_element.1);

    for _ in 0..40 {
        polymer_pairs = eval_polymer_2(&polymer_pairs, &rules);
        //println!("Step: {}", step);
    }

    let element_counts = count_elements_2(&polymer_pairs, first_char ,last_char);

    let max_element = element_counts.iter()
        .fold(('$', 0), |(mk, mv), (k, v)| {
            if *v > mv {
                (*k, *v)
            } else {
                (mk, mv)
            }
        });

    let min_element = element_counts.iter()
        .fold(('$', usize::MAX), |(mk, mv), (k, v)| {
            if *v < mv {
                (*k, *v)
            } else {
                (mk, mv)
            }
        });

    println!("After 10 steps:");
    println!("Most common element: {:?}", max_element);
    println!("Least common element: {:?}", min_element);

    println!("Result: {}", max_element.1 - min_element.1);



}
