use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn task_1(){
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let bags = contents.split("\n");

    let mut sum = 0;
    for i in bags{
        let s = i.split_at(i.len()/2);
        let a: HashSet<char> = HashSet::from_iter(s.0.chars());
        let b: HashSet<char> = HashSet::from_iter(s.1.chars());
        let diff = a.intersection(&b);
        for j in diff{
            if j.is_uppercase() {
                sum += (j.clone() as i32) - ('A' as i32) + 27;
            } else {
                sum += (j.clone() as i32) - ('a' as i32) + 1;
            }
        }
    }
    println!("{}", sum);

}