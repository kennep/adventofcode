use std::io::{stdin,BufRead};
use std::fmt;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SnailNumberParser;

#[derive(Clone,Eq,PartialEq)]
struct SnPair {
    left: Box<SnailNumber>,
    right: Box<SnailNumber>
}

#[derive(Clone,Eq,PartialEq)]
enum SnailNumber {
    Pair(SnPair),
    Number(i32)
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailNumber::Pair(p) => write!(f, "[{},{}]", p.left, p.right),
            SnailNumber::Number(n) => write!(f, "{}", n)
        }
    }
}

#[derive(Debug)]
enum FlatSn {
    StartPair,
    Number(i32),
    Sep,
    EndPair
}

impl fmt::Display for FlatSn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlatSn::StartPair => write!(f, "["),
            FlatSn::Number(n) => write!(f, "{}", n),
            FlatSn::Sep => write!(f, ","),
            FlatSn::EndPair => write!(f, "]")
        }
    }
}

struct Flattened {
    elems: Vec<FlatSn>
}

impl fmt::Display for Flattened {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            self.elems.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("")
        )
    }
}

impl SnailNumber {
    fn add(&self, other: &SnailNumber) -> SnailNumber {
        SnailNumber::Pair(SnPair{
            left: Box::new(self.clone()),
            right: Box::new(other.clone())
        }).reduce()
    }

    fn flatten(&self) -> Flattened {
        let mut output: Vec<FlatSn> = Vec::new();
        self.flatten_impl(&mut output);
        Flattened{elems: output}
    }

    fn flatten_impl(&self, output: &mut Vec<FlatSn>) {
        match self {
            SnailNumber::Number(n) => output.push(FlatSn::Number(*n)),
            SnailNumber::Pair(p) => {
                output.push(FlatSn::StartPair);
                p.left.flatten_impl(output);
                output.push(FlatSn::Sep);
                p.right.flatten_impl(output);
                output.push(FlatSn::EndPair);
            }
        }
    }

    fn explode(&self) -> (SnailNumber, bool)
    {
        let mut flattened = self.flatten();
        let len = flattened.elems.len();
        let mut left_number_idx = -1;
        let mut right_number_idx = -1;
        let mut start_exploding_pair = -1;
        let mut end_exploding_pair = -1;
        let mut nesting_level = 0;
        for i in 0..len {
            match flattened.elems[i] {
                FlatSn::Number(_) if start_exploding_pair == -1 => left_number_idx = i as i32,
                FlatSn::Number(_) if end_exploding_pair != -1 => right_number_idx = i as i32,
                FlatSn::Number(_) => (),
                FlatSn::StartPair => {
                    nesting_level += 1;
                    if nesting_level >= 5 && start_exploding_pair == -1 {
                        start_exploding_pair = i as i32;
                    }
                },
                FlatSn::EndPair => {
                    if nesting_level == 5 && end_exploding_pair == -1 {
                        end_exploding_pair = i as i32;
                    }
                    nesting_level -= 1;
                }
                FlatSn::Sep => ()
            }
            if right_number_idx != -1 {
                break;
            }
        }
        if start_exploding_pair != -1 {
            if let FlatSn::Number(left) = flattened.elems[start_exploding_pair as usize + 1] {
                if left_number_idx != -1 {
                    if let FlatSn::Number(existing_left) = flattened.elems[left_number_idx as usize] {
                        flattened.elems[left_number_idx as usize] = FlatSn::Number(existing_left + left);
                    }
                }
            }
            if let FlatSn::Number(right) = flattened.elems[end_exploding_pair as usize - 1] {
                if right_number_idx != -1 {
                    if let FlatSn::Number(existing_right) = flattened.elems[right_number_idx as usize] {
                        flattened.elems[right_number_idx as usize] = FlatSn::Number(existing_right + right);
                    }
                }                
            }
            flattened.elems.splice(start_exploding_pair as usize..=end_exploding_pair as usize, [FlatSn::Number(0)]);
            return (parse(flattened.to_string()), true)
        }
        (self.clone(), false)
    }

    fn split(&self) -> (SnailNumber, bool)
    {
        match self {
            SnailNumber::Number(n) if *n >= 10 => {
                let left = n / 2; // integer division, rounds down
                (SnailNumber::Pair(SnPair{
                    left: Box::new(SnailNumber::Number(left)),
                    right: Box::new(SnailNumber::Number(n - left))
                }), true)
            },
            SnailNumber::Number(_) => (self.clone(), false),
            SnailNumber::Pair(p) => {
                let (left_result, was_split) = p.left.split();
                if was_split {
                    (SnailNumber::Pair(SnPair{
                        left: Box::new(left_result),
                        right: p.right.clone()
                    }), true)
                } else {
                    let (right_result, was_split) = p.right.split();
                    (SnailNumber::Pair(SnPair{
                        left: Box::new(left_result),
                        right: Box::new(right_result)
                    }), was_split)
                }
            }
        }
    }

    fn reduce(&self) -> SnailNumber
    {
        let mut result = self.clone();
        loop {
            let (exploded_result, was_exploded) = result.explode();
            if was_exploded {
                result = exploded_result;
                continue;
            }
            let (split_result, was_split) = exploded_result.split();
            if was_split {
                result = split_result;
                continue;
            }
            return result;
        }
    }

    fn magnitude(&self) -> i32
    {
        match self {
            SnailNumber::Number(n) => *n,
            SnailNumber::Pair(p) => {
                3*p.left.magnitude() + 2*p.right.magnitude()
            }
        }
    }
}

fn parse_rule(input: Pair<Rule>) -> SnailNumber {
    match input.as_rule() {
        Rule::snail_number => {
            let mut it = input.into_inner();
            SnailNumber::Pair(SnPair{
                left: Box::new(parse_rule(it.next().unwrap())),
                right: Box::new(parse_rule(it.next().unwrap()))
            })
        },
        Rule::number => {
            SnailNumber::Number(input.as_str().parse::<i32>().unwrap())
        }
        Rule::digit => {
            SnailNumber::Number(input.as_str().parse::<i32>().unwrap())
        }
    }
}

fn parse(input: String) -> SnailNumber{
    let pairs = SnailNumberParser::parse(Rule::snail_number, &input)
        .expect("Parse failed")
        .next().unwrap();

    parse_rule(pairs)
}

fn main() {
    let input: Vec<SnailNumber> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(parse)
        .collect();

    let input_copy = input.clone();

    let sum = input.into_iter().reduce(|a, b| a.add(&b)).unwrap();
    println!("Final magnitude of sum is: {}", sum.magnitude());

    let mut largest_magnitude: i32 = 0;
    let input_len = input_copy.len();
    for i in 0..input_len {
        let a = input_copy[i].clone();
        for j in 0..input_len {
            let b = input_copy[j].clone();
            let magnitude = a.add(&b).magnitude();
            if magnitude > largest_magnitude {
                largest_magnitude = magnitude;
            }
            let magnitude = b.add(&a).magnitude();
            if magnitude > largest_magnitude {
                largest_magnitude = magnitude;
            }
        }
    }
    println!("Largest magnitude of adding two numbers is: {}", largest_magnitude);

}

#[test]
fn test_explode() {
    let sn = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
    let (exploded, performed) = sn.explode();
    println!("{}", sn);
    println!("{}", exploded);
    assert_eq!(exploded.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    assert_eq!(performed, true);
}

#[test]
fn test_explode2() {
    let sn = parse("[[[[[9,8],1],2],3],4]".to_string());
    let (exploded, performed) = sn.explode();
    println!("{}", sn);
    println!("{}", exploded);
    assert_eq!(exploded.to_string(), "[[[[0,9],2],3],4]");
    assert_eq!(performed, true);
}

#[test]
fn test_explode3() {
    let sn = parse("[7,[6,[5,[4,[3,2]]]]]".to_string());
    let (exploded, performed) = sn.explode();
    println!("{}", sn);
    println!("{}", exploded);
    assert_eq!(exploded.to_string(), "[7,[6,[5,[7,0]]]]");
    assert_eq!(performed, true);
}

#[test]
fn test_explode4() {
    let sn = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string());
    let (exploded, performed) = sn.explode();
    println!("{}", sn);
    println!("{}", exploded);
    assert_eq!(exploded.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    assert_eq!(performed, true);
}

#[test]
fn test_no_explode() {
    let sn = parse("[7,[6,[5,[7,0]]]]".to_string());
    let (exploded, performed) = sn.explode();
    println!("{}", sn);
    println!("{}", exploded);
    assert_eq!(exploded.to_string(), "[7,[6,[5,[7,0]]]]");
    assert_eq!(performed, false);
}

#[test]
fn test_split() {
    let sn = parse("[[[[0,7],4],[15,[0,13]]],[1,1]]".to_string());
    let (split, performed) = sn.split();
    println!("{}", sn);
    println!("{}", split);
    assert_eq!(split.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    assert_eq!(performed, true);

    let (split, performed) = split.split();
    println!("{}", split);
    assert_eq!(split.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    assert_eq!(performed, true);

    let (split, performed) = split.split();
    println!("{}", split);
    assert_eq!(split.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    assert_eq!(performed, false);
}

#[test]
fn test_reduce() {
    let sn = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
    let reduced = sn.reduce();
    println!("{}", sn);
    println!("{}", reduced);
    assert_eq!(reduced.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn test_magnitude() {
    let sn = parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string());
    assert_eq!(sn.magnitude(), 3488);
}