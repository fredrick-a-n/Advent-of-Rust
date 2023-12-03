use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let values: Vec<&str> = contents.split("\n").collect();

    // numbers ((x0,x1),y)
    let mut numbers: HashMap<((i32,i32),i32), i32> = HashMap::new();
    let mut symbols: HashMap<(i32,i32), char> = HashMap::new();
    
    for (y, line) in values.iter().enumerate() {
        let mut num: Vec<i32> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num.push(c.to_digit(10).unwrap() as i32);
            } else {
                if c != '.'{
                    symbols.insert((x as i32,y as i32), c);
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

    let sum = numbers.iter()
        .filter(|(((x0, x), y),_)|{
            for i in *x0..*x {
                if  symbols.contains_key(&(i-1,*y)) ||
                    symbols.contains_key(&(i+1,*y)) ||
                    symbols.contains_key(&(i,*y-1)) ||
                    symbols.contains_key(&(i,*y+1)) ||
                    symbols.contains_key(&(i-1,*y-1)) ||
                    symbols.contains_key(&(i-1,*y+1)) ||
                    symbols.contains_key(&(i+1,*y-1)) ||
                    symbols.contains_key(&(i+1,*y+1)) 
                {
                    return true
                }
            }  
            return false
        })
        .fold(0, |acc ,(_,n)| acc+n);
    println!("Task 1: {:?}", sum);


} 
