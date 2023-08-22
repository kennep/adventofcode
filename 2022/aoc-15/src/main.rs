use std::{io::{stdin, BufRead}, cmp::{max, min}};
use regex::Regex;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
struct SensorBeacon
{
    sensor: Point,
    beacon: Point
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
struct HorizLine {
    start: i32,
    end: i32
}

impl SensorBeacon {
    fn mh_distance(&self) -> i32 {
        (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs()
    }
}

fn covered_lines(sb: &Vec<SensorBeacon>, y: i32) -> Vec<HorizLine>
{
    let mut cl: Vec<_> = sb.iter()
        .map(|sb| (sb.sensor, sb.mh_distance()))
        .filter(|(s, d)| (y - s.y).abs() < *d)
        .map(|(s, d)| {
            let dm = d - (y - s.y).abs();
            HorizLine{start: s.x - dm, end: s.x + dm}
        })
        .collect();
    
    cl.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    cl
}

fn collapse_lines(lines: &[HorizLine]) -> Vec<HorizLine> {
    lines.iter()
        .fold(Vec::new(),
        |mut hl, l| {
            let len = hl.len();
            if hl.is_empty() {
                hl.push(*l);
                return hl;
            }
            let last = hl[len - 1];
            if l.start > last.end + 1 {
                hl.push(*l);
                return hl;
            }

            if last.end > l.end {
                return hl;
            }

            hl[len - 1] = HorizLine{start: last.start, end: l.end};
            hl
        })
}

fn diff(lines: &[HorizLine], diff_with: HorizLine) -> Vec<HorizLine> {
    lines.iter()
        .filter(|l| l.end >= diff_with.start && l.start <= diff_with.end)
        .map(|l| HorizLine{start: max(diff_with.start, l.start), end: min(diff_with.end, l.end)})
        .collect()
}

/*        111111111122222222223333333333
0123456789012345678901234567890123456789
  ######
    #######
                   ########
             ###
       ##############
                        #####

  ###########################                      
*/


fn measure_lines(lines: &[HorizLine]) -> i32 {
    /* 
    let hs = lines.iter()
        .fold(HashSet::new(), |mut h, l| {
            for x in l.start..l.end {
                h.insert(x);
            }
            h
        });
    return hs.len() as i32;
    */
    let collapsed = collapse_lines(lines);
    return collapsed.iter().map(|h| h.end - h.start).sum::<i32>();
}

fn main() {
    let lines = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty());

    let regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let sensor_beacons: Vec<_> = lines
        .map(|l| {
            if let Some(caps) = regex.captures(&l) {
                return SensorBeacon {
                    sensor: Point {
                        x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    },
                    beacon: Point {
                        x: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        y: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    },
                }    
            }
            panic!("Unexpected input: {}", l);
        })
        .collect();

    //let y = 10;
    let y = 2000000;

    println!("{:?}", sensor_beacons);
    let lines = covered_lines(&sensor_beacons, y);
    println!("{:?}", lines);
    let col_lines = collapse_lines(&lines);
    println!("{:?}", col_lines);
    println!("Length at {} where beacon cannot be: {}", y, measure_lines(&lines));

    //let xl = HorizLine{start: 0, end: 20};
    //for y in 0..= 20 {
    let xl = HorizLine{start: 0, end: 4000000};
    for y in 0..= 4000000 {
            let lines = covered_lines(&sensor_beacons, y);
        let col_lines = diff(&collapse_lines(&lines), xl);
        if col_lines.len() >= 2 {
            println!("Col-lines: {:?}", col_lines);    
            let x = col_lines[0].end + 1;        
            println!("Found beacon at {}, {}", x, y);
        }
    }
}
