use std::collections::HashMap;
use std::fs;
use std::path::Path;


// A, X = rock, B, Y = paper, C, Z = scissors
fn initiate_map() -> HashMap<&'static str, i32> {
    let mut win_pattern:HashMap<&str, i32> = HashMap::new();
    win_pattern.insert("A Y", 6+2);
    win_pattern.insert("A X", 3+1);
    win_pattern.insert("A Z", 0+3);
    win_pattern.insert("B Y", 3+2);
    win_pattern.insert("B X", 0+1);
    win_pattern.insert("B Z", 6+3);
    win_pattern.insert("C Y", 0+2);
    win_pattern.insert("C X", 6+1);
    win_pattern.insert("C Z", 3+3);
    return win_pattern;
}

pub fn task_1(){
    let path = Path::new("./input1");
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let win_pattern = initiate_map();
    
    let mut points: i32 = 0;
    let temp = contents.split("\n");
    for i in temp{
        points += win_pattern.get(i).unwrap();
    }

    println!("{}", points);
}