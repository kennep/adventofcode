use std::io::{stdin, BufRead};

fn wrap(mut i: i64, limit: usize) -> usize
{
    //((a % b) + b) % b, 
    let l = (limit - 1) as i64;
    (((i % l) + l) % l) as usize

    /* 
    while i < 0 {
        i += (limit - 1) as i64;
    }
    while i >= limit as i64 {
        i -= (limit as i64 - 1) as i64;
    }
    i as usize
    */
}

fn mix(input: &Vec<(usize, i64)>, output: &mut Vec<(usize, i64)>, len: usize)
{
    for &(idx, num) in input.iter() {
        let src_idx = output.iter().position(|&(i, n)| i == idx).unwrap();
        let target_idx = wrap(src_idx as i64 + num, len);
        output.remove(src_idx);
        output.insert(target_idx, (idx, num));
    }
}

fn main() {
    let input: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .enumerate()
        .collect();

    let len = input.len();
    let mut output = input.clone();
    mix(&input, &mut output, len);
    let result = output.iter().map(|o| o.1).collect::<Vec<_>>();
    println!("Decrypted file: {:?}", result);
    let z_idx = result.iter().position(|&n| n == 0).unwrap();
    println!("Position of 0: {}", z_idx);
    let mut iter = result.iter().cycle().skip(z_idx + 1);
    let mut sum = 0;
    for i in 0..3 {
        let idx = 1000 + i*1000;
        let num = *iter.nth(999).unwrap();
        sum += num;
        println!("Number at index {} is {}", idx, num);
    }
    println!("Sum: {}", sum);

    let input: Vec<_> = input.iter().map(|&(i, n)| (i, n * 811589153)).collect();
    let mut output = input.clone();
    for i in 0..10 {
        mix(&input, &mut output, len);
        let result = output.iter().map(|o| o.1).collect::<Vec<_>>();
        println!("Decrypted file after {} rounds: {:?}", i + 1, result);    
    }
    let result = output.iter().map(|o| o.1).collect::<Vec<_>>();
    let z_idx = result.iter().position(|&n| n == 0).unwrap();
    println!("Position of 0: {}", z_idx);
    let mut iter = result.iter().cycle().skip(z_idx + 1);
    let mut sum = 0;
    for i in 0..3 {
        let idx = 1000 + i*1000;
        let num = *iter.nth(999).unwrap();
        sum += num;
        println!("Number at index {} is {}", idx, num);
    }
    println!("Sum: {}", sum);
    
    
}
