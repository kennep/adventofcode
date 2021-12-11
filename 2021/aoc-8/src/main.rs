use std::io::{stdin,BufRead};
use itertools::Itertools;

struct Segments {
    signals: [bool; 7]
}

impl Segments {
    fn new() -> Segments {
        return Segments { signals: [false; 7]}
    }

    fn char_to_pos(ch: char) -> usize {
        match ch {
            'a'|'b'|'c'|'d'|'e'|'f'|'g' => (ch as u32 - 'a' as u32) as usize,
            _ => panic!("Invalid segment char {}", ch) 
        }
    }

    fn from_string(input: &str) -> Segments {
        let mut segments = Segments::new();

        for ch in input.chars() {
            segments.signals[Segments::char_to_pos(ch)] = true;
        }

        segments
    }

    fn map(&self, mapping: &[usize]) -> Segments
    {
        let mut segments = Segments::new();

        for (idx, entry) in mapping.iter().enumerate() {
            segments.signals[*entry] = self.signals[idx]
        }

        segments
    }

    /*

     0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

    */
    fn decode(&self) -> Option<usize>
    {
        match self.signals {
            [true, true, true, false, true, true, true] => Some(0),
            [false, false, true, false, false, true, false] => Some(1),
            [true, false, true, true, true, false, true] => Some(2),
            [true, false, true, true, false, true, true] => Some(3),
            [false, true, true, true, false, true, false] => Some(4),
            [true, true, false, true, false, true, true] => Some(5),
            [true, true, false, true, true, true, true] => Some(6),
            [true, false, true, false, false, true, false] => Some(7),
            [true, true, true, true, true, true, true] => Some(8),
            [true, true, true, true, false, true, true] => Some(9),
            _ => None
        }
    }

    fn is_legal(&self) -> bool{
        match self.decode() {
            Some(_) => true,
            None => false
        }
    }

    fn num_segments_on(&self) -> usize {
        self.signals.iter().filter(|s| **s).count()
    }
}

struct Measurement {
    wires: Vec<Segments>,
    digits: Vec<Segments>
}

impl Measurement {    
    fn parse_segments(input: &str) -> Vec<Segments> {
        input.split(" ").map(|i| Segments::from_string(i)).collect()
    }

    fn parse_measurement_line(input: &str) -> Measurement {
        let mut split_input = input.split(" | ");
        Measurement {
            wires: Measurement::parse_segments(split_input.next().expect("Expected two fields (could not get wires)")),
            digits: Measurement::parse_segments(split_input.next().expect("Expected two fields (could not get wires)"))
        }
    }

}

fn main() {
    let input: Vec<Measurement> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| Measurement::parse_measurement_line(&l))
        .collect();

    let num_1478: usize = input.iter()
        .map(|m| m.digits.iter().map(|s| match s.num_segments_on() {
            2 => 1, // one
            4 => 1, // four
            3 => 1, // seven
            7 => 1, // eight
            _ => 0 // other digit
        }).sum::<usize>()).sum();

    println!("Number of times 1, 4, 7, 8 appear: {}", num_1478);


    let mut sum = 0;
    for entry in input.iter() {
        let mut found = false;
        for mapping in (0 as usize..7 as usize).permutations(7) {
            //println!("{:?}", mapping);
            if entry.wires.iter().filter(|s| s.map(&mapping).is_legal()).count() == entry.wires.len() {
                let decoded: Vec<usize> = entry.digits.iter().map(|s| s.map(&mapping).decode().unwrap()).collect();
                let decoded_num = decoded[0] * 1000 + decoded[1] * 100 + decoded[2] * 10 + decoded[3];
                println!("Found valid mapping {:?}. Digit readout: {}", mapping, decoded_num);
                found = true;
                sum += decoded_num;
                break;
            }
        }    
        if !found {
            panic!("No mapping found for entry");
        }  
    }
    println!("Sum: {}", sum);
}
