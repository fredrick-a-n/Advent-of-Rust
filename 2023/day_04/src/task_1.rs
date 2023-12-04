use std::fs;
use std::path::Path;

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let rows: Vec<(Vec<i32>, Vec<i32>)> = contents.split("\n").map(|row| 
        row.split(":").last().unwrap().replace("  ", " ").split("|").map(|x| 
            x.trim().split(" ").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>()
    ).map(|x| (x[0].clone(), x[1].clone())).collect::<Vec<(Vec<i32>, Vec<i32>)>>();
    
    let point: i32 = rows.iter().map(|(a,b)|
        a.iter().fold(0, |acc, x| if b.contains(x) {if acc == 0 {1} else {acc*2}} else {acc})
    ).sum();
    
    println!("Task 1: {}", point);

} 
