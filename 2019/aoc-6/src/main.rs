use std::io::{stdin, BufRead};
use std::collections::HashMap;
use std::rc::Rc;

struct Object {
    name: String,
    orbited_by: Vec<Rc<Object>>,
    orbits: Vec<Rc<Object>>
}

impl Object {
    fn new(name: &str) -> Object {
        Object{ name: String::from(name), orbited_by: Vec::new(), orbits: Vec::new() }
    }
}

fn main() {
    let input: Vec<(String, String)> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.split(')').map(|s| String::from(s)).collect::<Vec<String>>())
        .map(|mut s| (s.remove(0), s.remove(0)))
        .collect();

    let mut objects: HashMap<String, Rc<Object>> = HashMap::new();

    for (s, d) in input.iter() {
        println!("{} ) {}", s, d);
        let mut src_obj = objects.entry(s.clone())
            .or_insert_with(|| Rc::new(Object::new(&s))).clone();
        let mut dst_obj = objects.entry(d.clone())
            .or_insert_with(|| Rc::new(Object::new(&d))).clone();
        src_obj.orbited_by.push(dst_obj);
        dst_obj.orbits.push(src_obj);
    }
}
