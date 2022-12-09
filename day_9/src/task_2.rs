use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn move_head(rope: &mut  Vec<(i32,i32)> , direction: &str){
    let mut new_head = rope.get(0).unwrap().clone();
    match direction {
        "R" => new_head.0 += 1,
        "U" => new_head.1 += 1,
        "L" => new_head.0 -= 1,
        "D" => new_head.1 -= 1,
        _ => panic!("Invalid input"),
    }
    rope[0] = new_head.clone();
    move_tail(rope, 1 );
}

fn move_tail(rope: &mut Vec<(i32,i32)>, i: usize){
    if i >= rope.len() {
        return;
    }
    let head = rope.get(i).unwrap().clone();
    let last = rope.get(i-1).unwrap();
    let diff = (last.0 - head.0, last.1 - head.1);
    if (diff.0.abs() > 1) || (diff.1.abs() > 1) {
        rope[i] = (head.0 + diff.0.signum(), head.1 + diff.1.signum());
        move_tail(rope, i+1);
    } 
}

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0,0); 10];
    visited.insert(rope.last().unwrap().clone());
    for l in contents.lines(){
        let ke: Vec<&str> = l.split(" ").collect();
        for _i in 0..(ke.get(1).unwrap().parse::<i32>().unwrap()) {
            move_head(&mut rope, ke.get(0).unwrap());
            visited.insert(rope.last().unwrap().clone());
        }
    }
    println!("{}", visited.len())
}