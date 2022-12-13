use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::io::{stdin, BufRead};

#[derive(PartialEq, Clone, Ord, Eq)]
enum Item {
    List(Vec<Item>),
    Value(i32),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(item) => {
                f.write_str("[")?;
                let mut first = true;
                for i in 0..item.len() {
                    if first {
                        first = false;
                    } else {
                        f.write_str(",")?;
                    }
                    std::fmt::Display::fmt(&item[i], f)?;
                }
                f.write_str("]")?;
            }
            Item::Value(v) => std::fmt::Display::fmt(&v, f)?,
        }
        Ok(())
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::List(s), Item::List(o)) => {
                for i in 0..s.len() {
                    if i >= o.len() {
                        return Some(Ordering::Greater);
                    }
                    let item_result = s[i].partial_cmp(&o[i]);
                    if let Some(ordering) = item_result {
                        match ordering {
                            Ordering::Less | Ordering::Greater => return Some(ordering),
                            Ordering::Equal => (),
                        }
                    }
                }
                if s.len() == o.len() {
                    return Some(Ordering::Equal);
                } else {
                    return Some(Ordering::Less);
                }
            }
            (Item::List(s), Item::Value(o)) => s.partial_cmp(&vec![Item::Value(*o)]),
            (Item::Value(s), Item::List(o)) => vec![Item::Value(*s)].partial_cmp(o),
            (Item::Value(s), Item::Value(o)) => s.partial_cmp(o),
        }
    }
}

fn parse_list_items(chars: &mut impl Iterator<Item = char>) -> Item {
    let mut result_items: Vec<Item> = Vec::new();
    let mut current_value: Option<i32> = None;
    loop {
        let inp = chars.next();
        match inp {
            Some(',') => {
                if let Some(val) = current_value {
                    result_items.push(Item::Value(val));
                    current_value = None;
                }
            }
            Some('[') => result_items.push(parse_list_items(chars)),
            Some(']') => {
                if let Some(val) = current_value {
                    result_items.push(Item::Value(val));
                }
                return Item::List(result_items);
            }
            Some(c) if c.is_digit(10) => {
                let digit_val = c.to_digit(10).unwrap() as i32;
                match current_value {
                    Some(val) => current_value = Some(val * 10 + digit_val),
                    None => current_value = Some(digit_val),
                }
            }
            Some(c) => panic!("Unexpected char in input: {}", c),
            None => panic!("Unexpected end of input when reading item"),
        };
    }
}

fn parse_list(chars: &mut impl Iterator<Item = char>) -> Item {
    match chars.next() {
        Some('[') => parse_list_items(chars),
        Some(c) => panic!("Unexpected char in input, expected [: {}", c),
        None => panic!("Unexpected end of input when reading start of list"),
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let linepairs = lines.split(|l| l.is_empty());

    let pairs: Vec<_> = linepairs
        .map(|s| {
            let p1 = parse_list(&mut s[0].chars());
            let p2 = parse_list(&mut s[1].chars());
            (p1, p2)
        })
        .collect();

    let pairs_in_right_order: Vec<_> = pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .collect();
    println!("Pairs in right order: {:?}", pairs_in_right_order);

    println!(
        "Sum of indices of pairs in right order: {}",
        pairs_in_right_order.iter().sum::<usize>()
    );

    let mut packets: Vec<Item> = pairs
        .iter()
        .flat_map(|(p1, p2)| vec![p1.clone(), p2.clone()])
        .map(|i| i.to_owned())
        .collect();

    let divider_packet_str = vec!["[[2]]", "[[6]]"];
    let mut divider_packets: Vec<_> = divider_packet_str
        .iter()
        .map(|s| parse_list(&mut s.chars()))
        .collect();

    packets.append(&mut divider_packets);

    packets.sort();

    println!("Sorted packets with dividers");
    for i in 0..packets.len() {
        println!("{}", packets[i]);
    }

    let divider_idxs: Vec<_> = packets
        .iter()
        .enumerate()
        .map(|(i, s)| (i, s.to_string()))
        .filter(|(_, s)| divider_packet_str.contains(&&s[..]))
        .map(|(i, _)| i + 1)
        .collect();
    println!("Divider indexes {:?}", divider_idxs);
    println!(
        "Distress signal decoder key: {}",
        divider_idxs.iter().fold(1, |a, v| a * v)
    );
}
