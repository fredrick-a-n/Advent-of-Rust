use std::fs;
use std::path::Path;


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let values: Vec<&str> = contents.split("\n").collect();
    let values: Vec<Vec<&str>> = values.iter()
        .map(|x| x.split(":").last().unwrap().split(";").collect::<Vec<&str>>())
        .collect();
    //(R,G,B)
    let row_mins:Vec<(i32,i32,i32)> = values.iter()
        .map(|row| 
            row.iter()
                .map(|x| x.split(",").collect::<Vec<&str>>())
                .map(|x| x.iter()
                    .map(|x| x.trim().split(" ").collect::<Vec<&str>>())
                    .map(|x| (x.first().unwrap().parse::<i32>().unwrap().clone(), x.last().unwrap().clone()))
                    .fold((0,0,0),|acc,e| 
                        match e.1.to_string().as_str()  {
                            "green" => (acc.0,acc.1+e.0,acc.2),
                            "red" => (acc.0+e.0,acc.1,acc.2),
                            "blue" => (acc.0,acc.1,acc.2+e.0),
                            _ => panic!("Should not happen")
                        }
                    )
                )
                .fold((0,0,0),|acc,e| 
                    (if e.0>acc.0 {e.0} else {acc.0}, if e.1>acc.1 {e.1} else {acc.1}, if e.2>acc.2 {e.2} else {acc.2})
                )
        )
        .collect();
    let row_mul = row_mins.iter()
        .fold(0,|acc, e| acc + (e.0*e.1*e.2));
    println!("Task 2: {}", row_mul);
} 
