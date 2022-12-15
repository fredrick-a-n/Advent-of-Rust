use std::fs;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

struct Sensor{
    sensor: (i64,i64),
    range: i64,
}

impl Sensor {
    pub fn new(sensor: (i64,i64), beacon: (i64,i64)) -> Self {
        let range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        Self{
            sensor,
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

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let sensors: Vec<Sensor> = contents.lines().map(|x| x.parse().unwrap()).collect();

    const MIN_XY: i64 = 0;
    const MAX_XY: i64 = 4000000;

    let mut distress = (0,0);

    let mut explore = vec![((MIN_XY,MIN_XY), (MAX_XY, MAX_XY))];

    while let Some((min,max)) = explore.pop() {
        if min == max {
            if sensors.iter().all(|s| (s.sensor.0 - min.0).abs() + (s.sensor.1 - min.1).abs() > s.range) {
                distress = min;
                break;
            }
        } else {
            let mid = ((min.0 + max.0) / 2, (min.1 + max.1) / 2);
            let divide = [
                (min, mid),
                ((mid.0 + 1, min.1), (max.0, mid.1)),
                ((min.0, mid.1 + 1), (mid.0, max.1)),
                ((mid.0 + 1, mid.1 + 1), max),
            ];
            for d in divide.iter() {
                if sensors.iter().all(|s| {
                    [(min.0, min.1), (min.0, max.1), (max.0, min.1), (max.0, max.1)].iter()
                        .map(|corner| (corner.0 - s.sensor.0).abs() + (corner.1 - s.sensor.1).abs())
                        .max().unwrap() > s.range
                    })
                    && !(d.0.0 > d.1.0 || d.0.1 > d.1.1){
                    explore.push(*d);
                }
            }
        }
    }
    
    println!("{}", distress.0 * 4000000 + distress.1);
}

