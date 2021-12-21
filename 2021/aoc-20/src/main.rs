use std::io::{stdin,BufRead};
use std::collections::HashMap;
use std::cell::RefCell;

trait ImageData {
    fn get(&self, x: i32, y: i32) -> u8;
}

impl ImageData for Vec<Vec<u8>>
{
    fn get(&self, x: i32, y: i32) -> u8 {
        if y >= 0 && y < self.len() as i32 {
            let row = &self[y as usize];
            if x >= 0 && x < row.len() as i32{
                return row[x as usize] 
            }
        }
        0
    }
}

struct ImageProcessor {
    enhancement_algo: Vec<u8>,    
    image: Box<dyn ImageData>,
    cache: RefCell<HashMap<(i32, i32), u8>>
}

impl ImageProcessor {
    fn new(enhancement_algo: Vec<u8>, image: Box<dyn ImageData>) -> ImageProcessor {
        ImageProcessor{
            enhancement_algo,
            image,
            cache: RefCell::new(HashMap::new())
        }
    }
}

impl ImageData for ImageProcessor {
    fn get(&self, x: i32, y: i32) -> u8 {
        *self.cache.borrow_mut().entry((x, y)).or_insert_with(|| {
            let mut idx: usize = 0;
            for sy in y - 1 ..= y + 1 {
                for sx in x - 1 ..= x + 1 {
                    idx <<= 1;
                    idx += self.image.get(sx, sy) as usize;
                }
            }
            return self.enhancement_algo[idx];
        })
    }
}

fn to_bit(c: char) -> u8
{
    match c {
        '#' => 1,
        _ => 0
    }
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .collect();

    let mut sections = lines[..].split(|l| l.is_empty());

    let enhancement_algo: Vec<u8> = sections.next().unwrap().join("")
        .chars().map(to_bit).collect();

    let image: Vec<Vec<u8>> = sections.next().unwrap().into_iter()
        .map(|l| l.chars().map(to_bit).collect::<Vec<u8>>())
        .collect();

    let mut image_processor = ImageProcessor::new(enhancement_algo.clone(), Box::new(image));
    for _ in 0..49 {
        image_processor = ImageProcessor::new(enhancement_algo.clone(), Box::new(image_processor));
    }

    let mut sum: i32 = 0;
    for y in -100..300 {
        println!("{}", y);
        for x in -100..300 {
            sum += image_processor.get(x, y) as i32;
            /*print!("{}", match second_pass.get(x, y) {
                1 => '#',
                _ => '.'
            });*/
        }
    }

    println!("Lit pixels: {}", sum);
}
