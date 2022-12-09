use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn move_head(head: (i32,i32), direction: &str) -> (i32,i32){
    let mut new_head = head.clone();
    match direction {
        "R" => new_head.0 += 1,
        "U" => new_head.1 += 1,
        "L" => new_head.0 -= 1,
        "D" => new_head.1 -= 1,
        _ => panic!("Invalid input"),
    }
    return new_head;
}

fn move_tail(head: (i32,i32), last_head: (i32,i32), tail: (i32,i32)) -> (i32,i32){
    if (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1 {
        return last_head;
    } else{
        return tail;
    }
}

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0,0);
    let mut tail = (0,0);
    visited.insert(tail.clone());
    for l in contents.lines(){
        let ke: Vec<&str> = l.split(" ").collect();
        for _i in 0..(ke.get(1).unwrap().parse::<i32>().unwrap()) {
            let new_head = move_head(head, ke.get(0).unwrap());
            tail = move_tail(new_head, head, tail);
            head = new_head;
            visited.insert(tail.clone());
        }
    }
    println!("{}", visited.len())
}