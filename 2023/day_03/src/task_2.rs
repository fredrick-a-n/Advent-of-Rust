use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let values: Vec<&str> = contents.split("\n").collect();

    // numbers ((x0,x1),y)
    let mut numbers: HashMap<((i32,i32),i32), i32> = HashMap::new();
    let mut gears: HashMap<(i32,i32), char> = HashMap::new();
    
    for (y, line) in values.iter().enumerate() {
        let mut num: Vec<i32> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num.push(c.to_digit(10).unwrap() as i32);
            } else {
                if c == '*'{
                    gears.insert((x as i32,y as i32), c);
                }
                if num.len() > 0 {
                    let mut n = 0;
                    for i in 0..num.len() {
                        n += num[i] * 10_i32.pow((num.len() - i - 1) as u32);
                    }
                    numbers.insert(((x as i32 - num.len() as i32, x as i32), y as i32), n);
                    num.clear();
                }
            }
        }
        if num.len() > 0 {
            let mut n = 0;
            for i in 0..num.len() {
                n += num[i] * 10_i32.pow((num.len() - i - 1) as u32);
            }
            numbers.insert(((line.len() as i32 - num.len() as i32, line.len() as i32), y as i32), n);
        }
    }

    let sum = gears.iter()
        .map(|((x, y),_)|{
            numbers.iter().filter(|(((x0, x1), y1),_)|{
                for i in *x0..*x1 {
                    if i-1 <= *x && *x <= i+1 && y1-1 <= *y && *y <= y1+1
                    {
                        return true
                    }
                }
                return false
            }).map(|(_,n)| *n).collect::<Vec<i32>>()
        })
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .fold(0, |acc ,n| acc+n);
    println!("Task 2: {:?}", sum);


} 
