use std::fs;
use std::path::Path;

pub fn task_1() {
    println!("Task 1: {}", fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file")
        .split("\n").map(|row| 
        row.split(":").last().unwrap().replace("  ", " ").split("|").map(|x| 
            x.trim().split(" ").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>()
        ).map(|x| (x[0].clone(), x[1].clone())).map(|(a,b)|
            a.iter().fold(0, |acc, x| if b.contains(x) {if acc == 0 {1} else {acc*2}} else {acc})
        ).sum::<i32>())
} 