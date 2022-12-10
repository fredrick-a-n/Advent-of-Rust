use std::fs;
use std::path::Path;
use regex::Regex;


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let c = contents.replace("addx","noop\naddx");
    let mut total_signal = 0;
    let mut cycle = 0;
    let mut x = 1;
    let re= r"(noop|addx [-+]?\d+)".to_owned();    
    for cap in Regex::new(&re).unwrap().captures_iter(&c) {
        cycle += 1;
        if cycle%40 == 20{
            total_signal += cycle * x;
        }
        match cap[1].split_at(4).0 {
            "addx" => x += cap[1].split_at(5).1.parse::<i32>().unwrap(),
            "noop" => (),
            _ => panic!("Invalid Input"),
        }
        if cycle > 220 {
            break;
        }
    }
    println!("{}", total_signal);
    
}