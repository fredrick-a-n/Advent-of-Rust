use std::fs;
use std::path::Path;

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut out = 0;
    for i in 0..contents.len() {
        if check_unique(&contents[i..(i+4)]){
            out = i.clone() + 4;
            break;
        }
    }
    println!("{}", out);
}

fn check_unique(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    for i in 0..(s.len()-1){
        for j in (i+1)..s.len(){
            if c.get(i) == c.get(j){
                return false;
            }
        }
    }
    return true;
} 