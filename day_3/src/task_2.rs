use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn task_2(){
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let bags: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;
    for i in 0..(bags.len()/3){
        let a: HashSet<char> = HashSet::from_iter(bags.get(i * 3).unwrap().chars());
        let b: HashSet<char> = HashSet::from_iter(bags.get(i * 3 + 1).unwrap().chars());
        let c: HashSet<char> = HashSet::from_iter(bags.get(i * 3 + 2).unwrap().chars());
        let diff = a.iter().filter(|k| b.contains(k)).filter(|k| c.contains(k));
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