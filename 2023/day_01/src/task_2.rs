use std::fs;
use std::path::Path;


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let table = vec![("one",1 ),("two",2),("three",3),("four",4),("five",5),("six",6),("seven",7),("eight",8),("nine",9)];
    let table_rev = vec![("eno",1 ),("owt",2),("eerht",3),("ruof",4),("evif",5),("xis",6),("neves",7),("thgie",8),("enin",9)];

    let nums: Vec<u32> = values.iter().map(|x| {
        let x = x.to_string();
        let mut i = 0;
        let mut first = 0;
        let mut last = 0;

        while i < x.len() {
            if x.chars().nth(i).unwrap().is_numeric() {
                first = x.chars().nth(i).unwrap().to_digit(10).unwrap();
                break;
            }
            let mut k = 0;
            while k < table.len() {
                let j = table[k].0.len()+i;
                if j < x.len()+1 {
                    if table[k].0 == &x[i..j] {
                        first = table[k].1;
                        break;
                    }
                }
                k += 1;
            }
            if first != 0 {
                break;
            }
            i += 1;
        }
        let mut i = 0;
        let x_rev = x.chars().rev().collect::<String>();
        while i < x_rev.len() {
            if x_rev.chars().nth(i).unwrap().is_numeric() {
                last = x_rev.chars().nth(i).unwrap().to_digit(10).unwrap();
                break;
            }
            let mut k = 0;
            while k < table_rev.len() {
                let j = table_rev[k].0.len()+i;
                if j < x_rev.len()+1 {
                    if table_rev[k].0 == &x_rev[i..j] {
                        last = table_rev[k].1;
                        break;
                    }
                }
                k += 1;
            }
            if last != 0 {
                break;
            }
            i += 1;
        }
        return first*10 + last;
    }).collect();
    println!("{}", nums.iter().sum::<u32>());
} 
