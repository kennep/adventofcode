extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MonkeyParser;

#[derive(Debug, Clone)]
enum Expression {
    Literal(i32),
    Variable,
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn value(&self, oldval: i32) -> i64 {
        let v = match self {
            Expression::Literal(l) => *l as i64,
            Expression::Variable => oldval as i64,
            Expression::Add(a, b) => a.value(oldval) as i64 + b.value(oldval) as i64,
            Expression::Multiply(a, b) => a.value(oldval) as i64 * b.value(oldval) as i64,
        };
        v
    }
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    cond_divisible_by: i32,
    if_true_target: usize,
    if_false_target: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Expression,
    test: MonkeyTest,
    inspection_count: usize,
}

fn parse_expression(mut expression: Pairs<Rule>) -> Expression {
    let pair = expression.next().unwrap();
    let expr = match pair.as_rule() {
        Rule::variable => Expression::Variable,
        Rule::numeric_literal => Expression::Literal(pair.as_str().parse::<i32>().unwrap()),
        _ => panic!("Invalid rule {:?}", pair),
    };

    match expression.next() {
        Some(p) => match p.as_str() {
            "*" => Expression::Multiply(Box::new(expr), Box::new(parse_expression(expression))),
            "+" => Expression::Add(Box::new(expr), Box::new(parse_expression(expression))),
            s => panic!("Invalid operator {}", s),
        },
        None => return expr,
    }
}

fn parse_test(mut test: Pairs<Rule>) -> MonkeyTest {
    let cond_divisible_by = test
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    let if_true_target = test
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let if_false_target = test
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    MonkeyTest {
        cond_divisible_by,
        if_true_target,
        if_false_target,
    }
}

fn parse_monkey(monkey: Pair<Rule>) -> Monkey {
    let mut rules = monkey.into_inner();
    let mut header_rules = rules.next().unwrap().into_inner();
    let id = header_rules
        .next()
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    let items_rules = rules.next().unwrap().into_inner();
    let items: Vec<_> = items_rules
        .map(|r| r.as_str().parse::<i32>().unwrap())
        .collect();
    let expression = rules
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .into_inner();
    let operation = parse_expression(expression);
    let test = parse_test(rules.next().unwrap().into_inner());

    Monkey {
        id,
        items,
        operation,
        test,
        inspection_count: 0,
    }
}

fn monkey_business<F>(mut monkeys: Vec<Monkey>, rounds: i32, operation: F)
where
    F: Fn(i64) -> i64,
{
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let moved_items: Vec<_> = monkeys[m]
                .items
                .iter()
                .map(|item| {
                    let mut worry = monkeys[m].operation.value(*item);
                    worry = operation(worry);
                    let target_monkey: usize;
                    if worry % (monkeys[m].test.cond_divisible_by as i64) == 0 {
                        target_monkey = monkeys[m].test.if_true_target;
                    } else {
                        target_monkey = monkeys[m].test.if_false_target;
                    }
                    (worry, target_monkey)
                })
                .collect();
            for (w, i) in moved_items {
                monkeys[i].items.push(w as i32);
            }
            monkeys[m].inspection_count += monkeys[m].items.len();
            monkeys[m].items.clear();
        }
    }

    let mut inspection_counts: Vec<_> = monkeys.iter().map(|m| m.inspection_count).collect();

    inspection_counts.sort();

    println!(
        "Insp count: {} {}",
        inspection_counts[monkeys.len() - 2],
        inspection_counts[monkeys.len() - 1]
    );

    let monkey_business =
        inspection_counts[monkeys.len() - 2] * inspection_counts[monkeys.len() - 1];

    println!("Monkey business: {}", monkey_business);
}

fn main() -> Result<(), String> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Read failed");

    let parse_result = MonkeyParser::parse(Rule::monkeys, &input);
    if let Err(e) = parse_result {
        eprintln!("{}", e.to_string());
        return Err("Parse error".to_owned());
    }

    let pairs = parse_result.unwrap();

    let monkeys: Vec<_> = pairs
        .map(|p| match p.as_rule() {
            Rule::monkey => parse_monkey(p),
            _ => panic!("Unexpected rule {:?}", p),
        })
        .collect();

    for (i, m) in monkeys.iter().enumerate() {
        if m.id != i as i32 {
            panic!("Unexpected ID {} for monkey at position {}", m.id, i);
        }
        println!("{:?}", m);
    }

    let gcd = monkeys.iter().fold(1, |a, m| a * m.test.cond_divisible_by) as i64;
    println!("GCD: {}", gcd);

    monkey_business(monkeys.clone(), 20, |w| w / 3);
    monkey_business(monkeys.clone(), 10000, |w| w % gcd);

    Ok(())
}
