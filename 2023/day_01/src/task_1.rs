use std::fs;
use std::path::Path;


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let nums: Vec<u32> = values.iter().map(|x| {
        let k: Vec<char> = x.clone().chars().filter(|c| c.is_numeric()).collect();
        println!("{:?}", k);

        let num = k.first().unwrap().to_digit(10).unwrap()*10 +  k.last().unwrap().to_digit(10).unwrap();
        num
    }).collect();
    println!("{:?}", nums);
    println!("{}", nums.iter().sum::<u32>());
} 
