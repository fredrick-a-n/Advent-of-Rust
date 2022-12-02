use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn initiate_map() -> HashMap<&'static str, i32> {
    let mut win_pattern:HashMap<&str, i32> = HashMap::new();
    win_pattern.insert("A Y", 3+1);
    win_pattern.insert("A X", 0+3);
    win_pattern.insert("A Z", 6+2);
    win_pattern.insert("B Y", 3+2);
    win_pattern.insert("B X", 0+1);
    win_pattern.insert("B Z", 6+3);
    win_pattern.insert("C Y", 3+3);
    win_pattern.insert("C X", 0+2);
    win_pattern.insert("C Z", 6+1);
    return win_pattern;
}

pub fn task_2(){
    let path = Path::new("./input1");
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let win_pattern:HashMap<&str, i32> = initiate_map();

    let mut points: i32 = 0;
    let temp = contents.split("\n");
    for i in temp{
        points += win_pattern.get(i).unwrap();
    }

    println!("{}", points);
}