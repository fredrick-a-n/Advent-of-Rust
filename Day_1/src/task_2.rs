use std::fs;
use std::path::Path;

pub fn task_2(){
    let path = Path::new("./input1");
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let mut values: Vec<&str> = contents.split("\n\n").collect();

    let mut k: Vec<i32> = Vec::new();
    for i in values{
        let w: Vec<&str> = i.split("\n").collect();
        let mut sum = 0;
        for j in w{
            if j != "" {
                sum += j.parse::<i32>().unwrap();
            }
        }
        k.push(sum);
    }   

    k.sort();
    let s = k.pop().unwrap_or_default() + k.pop().unwrap_or_default()+ k.pop().unwrap_or_default();
    println!("The sum of the top weights is {}", s)

}