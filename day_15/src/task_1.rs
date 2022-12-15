use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct Sensor{
    sensor: (i32,i32),
    beacon: (i32,i32),
    range: i32,
}

impl Sensor {
    pub fn new(sensor: (i32,i32), beacon: (i32,i32)) -> Self {
        let range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        Self{
            sensor,
            beacon,
            range,
        }
    }
}

impl FromStr for Sensor {
    type Err = ();    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sensor = (0,0);
        let mut beacon = (0,0);
        let re= r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)".to_owned();    
        for cap in Regex::new(&re).unwrap().captures_iter(s){
            sensor = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            beacon = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
        }

        return  Ok(Sensor::new(sensor, beacon));
    }
}

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let sensors: Vec<Sensor> = contents.lines().map(|x| x.parse().unwrap()).collect();
    
    const Y: i32 = 2000000;
    let lowest = sensors.iter().map(|x| x.sensor.0 - x.range).min().unwrap();
    let highest = sensors.iter().map(|x| x.sensor.0 + x.range).max().unwrap();

    let mut no_beacon: HashSet<(i32,i32)> = HashSet::new();
    for x in lowest..highest {
        for s in sensors.iter() {
            if (s.sensor.0 - x).abs() + (s.sensor.1 - Y).abs() <= s.range {
                no_beacon.insert((x,Y));
            }
        }
    }

    for s in sensors.iter() {
        no_beacon.remove(&s.beacon);
    }

    println!("{}", no_beacon.len());
}

