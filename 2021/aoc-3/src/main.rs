use std::io::{stdin, BufRead};

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(2).unwrap()).collect()
}

fn transpose(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let input_width = input[0].len();
    (0..input_width)
        .map(|p| input.iter().map(|e| e[p]).collect())
        .collect()
}

fn most_common(input: &Vec<u32>) -> u32 {
    let zeros = input.iter().filter(|i| **i == 0).count();
    let ones = input.iter().filter(|i| **i == 1).count();
    if zeros > ones { 0 } else { 1 }
}

fn to_decimal(input: &Vec<u32>) -> u32 {
    input.iter().rev().fold((1, 0), |(m, s), e| (m<<1, s + e*m)).1
}

fn extract_pos(input: &Vec<&Vec<u32>>, pos: usize) -> Vec<u32> {
    input.iter().map(|r| r[pos]).collect()
}

fn keep_elems_with_digits_in_pos<'a>(input: &Vec<&'a Vec<u32>>, pos: usize, digit: u32) -> Vec<&'a Vec<u32>> {
    input.iter().filter(|r| r[pos] == digit).map(|e| *e).collect()
}

fn keep_most_comon_digit_in_pos<'a>(input: &Vec<&'a Vec<u32>>, pos: usize) -> Vec<&'a Vec<u32>> {
    let mc = most_common(&extract_pos(input, pos));
    keep_elems_with_digits_in_pos(input, pos, mc)
}

fn keep_least_comon_digit_in_pos<'a>(input: &Vec<&'a Vec<u32>>, pos: usize) -> Vec<&'a Vec<u32>> {
    let mc = most_common(&extract_pos(input, pos));
    keep_elems_with_digits_in_pos(input, pos, 1 - mc)
}

fn main() {
    let input: Vec<Vec<u32>> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| parse(&l))
        .collect();

    let transposed = transpose(&input);

    let gamma:Vec<u32> = transposed.iter()
        .map(|e| most_common(e))
        .collect();
    let epsilon:Vec<u32> = transposed.iter()
        .map(|e| 1 - most_common(e))
        .collect();
    let gamma = to_decimal(&gamma);
    let epsilon = to_decimal(&epsilon);
    println!("A: gamma={}, epsilon={}, gxe={}", gamma, epsilon, gamma * epsilon);

    let mut value: Vec<&Vec<u32>> = input.iter().collect();
    let mut i = 0;
    while value.len() > 1 {
        value = keep_most_comon_digit_in_pos(&value, i);
        i = i + 1; 
    }
    let oxygen_rating = to_decimal(value[0]);

    let mut value: Vec<&Vec<u32>> = input.iter().collect();
    let mut i = 0;
    while value.len() > 1 {
        value = keep_least_comon_digit_in_pos(&value, i);
        i = i + 1; 
    }
    let co2_scrubber_rating = to_decimal(value[0]);

    println!("B: oxygen_rating={}, co2_scrubber_rating={}, oxc={}", oxygen_rating, co2_scrubber_rating, oxygen_rating*co2_scrubber_rating);
}
