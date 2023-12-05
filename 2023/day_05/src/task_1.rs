use std::fs;
use std::path::Path;

pub fn task_1() {
    let data: Vec<String> = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file")
        .split("\n\n").map(|x| x.to_string()).collect();
    let mut transforms: Vec<Vec<(i64,i64,i64)>> = Vec::new();
    let mut nums: Vec<i64> = data[0].split(" ").filter(|x| x.parse::<i64>().is_ok()).map(|x| x.parse::<i64>().unwrap()).collect();
    for i in 1..data.len() {
        let mut transform: Vec<(i64, i64, i64)> = Vec::new();
        for line in data[i].split("\n").skip(1) {
            let mut line = line.split(" ");
            let to = line.next().unwrap().parse::<i64>().unwrap();
            let from = line.next().unwrap().parse::<i64>().unwrap();
            let range = line.next().unwrap().parse::<i64>().unwrap();
            transform.push((to, from, range));
        }
        transforms.push(transform);
    }

    for transform in transforms {
        for num in nums.iter_mut() {
            for (to, from, range) in transform.iter() {
                if *num >= *from && *num < *from + *range {
                    *num = *to + (*num - *from);
                    break;
                }
            }
        }
    }

    let num = nums.iter().reduce(|a,b| if a < b {a} else {b}).unwrap();
    println!("Task 1: {}", num);

} 