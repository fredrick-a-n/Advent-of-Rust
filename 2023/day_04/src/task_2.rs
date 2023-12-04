use std::fs;
use std::path::Path;

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let rows: Vec<(Vec<i32>, Vec<i32>)> = contents.split("\n").map(|row| 
        row.split(":").last().unwrap().replace("  ", " ").split("|").map(|x| 
            x.trim().split(" ").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>()
    ).map(|x| (x[0].clone(), x[1].clone())).collect::<Vec<(Vec<i32>, Vec<i32>)>>();
    
    let mut cards: Vec<usize> = vec![1; rows.len()];

    for (i,(a,b)) in rows.iter().enumerate() {
        let ret = a.iter().fold(0, |acc, x| acc + if b.contains(x) {1} else {0});
        for j in (i+1)..=(i+ret) {
            cards[j] += cards[i];
        }
    }

    let point: usize = cards.iter().sum();
    
    println!("Task 2: {}", point);

} 