use std::fs;
use std::path::Path;
use regex::Regex;


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let c = contents.replace("addx","noop\naddx");
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut lines: Vec<String> = vec!["........................................".to_string(); 6];

    let re= r"(noop|addx [-+]?\d+)".to_owned();    
    for cap in Regex::new(&re).unwrap().captures_iter(&c) {
        cycle += 1;
        let curr = cycle%40;
        match cap[1].split_at(4).0 {
            "addx" => x += cap[1].split_at(5).1.parse::<i32>().unwrap(),
            "noop" => (),
            _ => panic!("Invalid Input"),
        }
        if (x - curr).abs() <= 1 {
            lines.get_mut((cycle/40) as usize).unwrap().replace_range((curr as usize)..((curr+1) as usize),"#");
        }

    }
    
    
    for l in lines {
        println!("{}", l);
    }

}