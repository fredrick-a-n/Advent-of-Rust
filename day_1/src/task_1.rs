use std::fs;
use std::path::Path;


pub fn task_1() {
    let path = Path::new("./input1");
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let values: Vec<&str> = contents.split("\n\n").collect();

    let mut max = 0;
    for i in values{
        let w: Vec<&str> = i.split("\n").collect();
        let mut sum = 0;
        for j in w{
            if j != "" {
                sum += j.parse::<i32>().unwrap();
            }
        }
        if sum > max{
            max = sum;
        }
    }   
    println!("The most weight is {}", max)

}
