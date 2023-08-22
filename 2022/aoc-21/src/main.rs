use std::io::{stdin, BufRead};
use std::collections::HashMap;

#[derive(Debug,Clone,Copy)]
enum MonkeyOperation {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug,Clone)]
enum MonkeyJob {
    Yell(i64),
    Operation(String, MonkeyOperation, String),
    Human
}

#[derive(Debug,Clone)]
struct Monkey {
    id: String,
    job: MonkeyJob,
    value: Option<i64>
}

fn monkey_value(monkey_id: &str, monkeys: &mut HashMap<String, Monkey>) -> Option<i64>
{
    let monkey = monkeys.get_mut(monkey_id).unwrap();
    if let Some(value) = monkey.value {
        return Some(value);
    }

    match monkey.job.clone() {
        MonkeyJob::Yell(value) => {
            monkey.value = Some(value);
            Some(value)
        }
        MonkeyJob::Operation(monkey_a, operation, monkey_b) => {
            let operand_a = monkey_value(&monkey_a.to_owned(), monkeys);
            let operand_b = monkey_value(&monkey_b.to_owned(), monkeys);
            match (operand_a, operand_b) {
                (Some(operand_a), Some(operand_b)) => {
                    let value = match operation {
                        MonkeyOperation::Add => operand_a + operand_b,
                        MonkeyOperation::Subtract => operand_a - operand_b,
                        MonkeyOperation::Multiply => operand_a * operand_b,
                        MonkeyOperation::Divide => operand_a / operand_b,
                    };
                    
                    let monkey = monkeys.get_mut(monkey_id).unwrap();
                    monkey.value = Some(value);
                    Some(value)        
                },
                _ => None
            }
        },
        MonkeyJob::Human => None
    }
    
}

fn pivot_tree(new_root: &str, monkeys: &HashMap<String, Monkey>, target_value: i64) -> HashMap<String, Monkey>
{
    let mut result: HashMap<String, Monkey> = monkeys.clone();

    let mut current_monkey = new_root.to_owned();
    loop {
        let current_node = monkeys.values().filter(|m| {
            if let MonkeyJob::Operation(a, _, b) = &m.job {
                return *a == current_monkey || *b == current_monkey
            }
            false
        }).next();
        if let None = current_node {
            panic!("Didn't find anything matching {}", current_monkey);
        }
        let current_node = current_node.unwrap();
        if let MonkeyJob::Operation(a, op, b) = &current_node.job {
            let (other_operand, current_is_first) = match (a, b) {
                (a, b) if a == &current_monkey => (b, true),
                (a, b) if b == &current_monkey => (a, false),
                _ => panic!("Expected operators to match current monkey")
            };
            if current_node.id == "root" {
                result.insert(current_monkey.clone(), Monkey{id: current_monkey.clone(), job: MonkeyJob::Yell(target_value), value: None});
                break;
            }
            let pivot_monkey = match op {
                MonkeyOperation::Add => Monkey{ id: current_monkey.clone(), job: MonkeyJob::Operation(current_node.id.clone(), MonkeyOperation::Subtract, other_operand.clone()), value: None},
                MonkeyOperation::Subtract => match current_is_first {
                    true => Monkey{ id: current_monkey.clone(), job: MonkeyJob::Operation(other_operand.clone(), MonkeyOperation::Add, current_node.id.clone()), value: None},
                    false => Monkey{ id: current_monkey.clone(), job: MonkeyJob::Operation(other_operand.clone(), MonkeyOperation::Subtract, current_node.id.clone()), value: None},
                },
                MonkeyOperation::Multiply => Monkey{ id: current_monkey.clone(), job: MonkeyJob::Operation(current_node.id.clone(), MonkeyOperation::Divide, other_operand.clone()), value: None},
                MonkeyOperation::Divide => match current_is_first {
                    true => Monkey{ id: current_monkey.clone(), job: MonkeyJob::Operation(other_operand.clone(), MonkeyOperation::Multiply, current_node.id.clone()), value: None},
                    false => Monkey{ id: current_monkey.clone(), job: MonkeyJob::Operation(other_operand.clone(), MonkeyOperation::Divide, current_node.id.clone()), value: None},
                },
            };
            result.insert(current_monkey.clone(), pivot_monkey);
            current_monkey = current_node.id.clone();
        } else {
            panic!("Expected monkey {} to contain an operation", current_monkey);
        }
    }

    result
}

fn main() {
    let monkeys: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|e| e.to_owned()).collect::<Vec<_>>())
        .map(|l| {
            let monkey_id = (l[0][..l[0].len() - 1]).to_owned();
            if l.len() == 4 {
                Monkey {
                    id: monkey_id,
                    job: MonkeyJob::Operation(
                        l[1].to_owned(),
                        match l[2].as_str() {
                            "*" => MonkeyOperation::Multiply,
                            "/" => MonkeyOperation::Divide,
                            "+" => MonkeyOperation::Add,
                            "-" => MonkeyOperation::Subtract,
                            _ => panic!("Invalid operation: {}", l[2])
                        },
                        l[3].to_owned()
                    ),
                    value: None
                }
            } else {
                Monkey {
                    id: monkey_id,
                    job: MonkeyJob::Yell(l[1].parse::<i64>().unwrap()),
                    value: None
                }
            }

        })
        .collect();

    let monkeys: HashMap<String, Monkey> = HashMap::from_iter(monkeys.into_iter().map(|m| (m.id.to_owned(), m)));
    println!("Root is yelling: {:?}", monkey_value("root", &mut monkeys.clone()));

    if let MonkeyJob::Operation(root_a, _, root_b) = &monkeys["root"].job {
        let mut monkeys_b = monkeys.clone();
        monkeys_b.get_mut("humn").unwrap().job = MonkeyJob::Human;

        let branch_a_value = monkey_value(root_a, &mut monkeys_b.clone());
        let branch_b_value = monkey_value(root_b, &mut monkeys_b.clone());
        println!("Value of A branch: {:?}", branch_a_value);
        println!("Value of B branch: {:?}", branch_b_value);

        let (fixed_branch, variable_monkey) = match (branch_a_value, branch_b_value) {
            (Some(_), Some(_)) => panic!("Expected one branch to be variable!"),
            (Some(a), None) => (a, root_b),
            (None, Some(b)) => (b, root_a),
            (None, None) => panic!("Expected one branch to be fixed!")
        };
        println!("Fixed branch value: {}. Variable branch top monkey: {}", fixed_branch, variable_monkey);
        let pivot = pivot_tree("humn", &monkeys_b, fixed_branch);
        let humn_value = monkey_value("humn", &mut pivot.clone());
        println!("Number human must yell: {:?}", humn_value);
    } else {
        panic!("Expected root to be an operation!");
    }


}
