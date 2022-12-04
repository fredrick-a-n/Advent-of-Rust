use std::fs;
use std::path::Path;
use regex::Regex;

pub fn task_2(){
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let re = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
    let mut count = 0;
    for cap in re.captures_iter(&contents) {
        let a = &cap[1].parse::<i32>().unwrap();
        let b = &cap[2].parse::<i32>().unwrap();
        let c = &cap[3].parse::<i32>().unwrap();
        let d = &cap[4].parse::<i32>().unwrap();
        if a <= d && c <= b{
            count += 1;
        } 
    }
    println!("{}", count);
}