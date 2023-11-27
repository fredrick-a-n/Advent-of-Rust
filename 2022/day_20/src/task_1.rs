use std::fs;
use std::path::Path;


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let mut nums: Vec<i32> = Vec::new();
    for l in contents.lines(){
        nums.push(l.parse().unwrap());
    }
    let mut output: Vec<usize> = (0..nums.len()).collect();
    for (i, x) in nums.iter().enumerate() {
        let pos = output.iter().position(|y| *y == i).unwrap();
        output.remove(pos);
        let new_index = (pos as i32 + x).rem_euclid(output.len() as i32) as usize;
        output.insert(new_index, i);

    }
    let zero = output.iter().position(|y| nums.iter().position(|x| x.clone() == 0 ).unwrap() == *y).unwrap();
    let coords = (nums[output[(zero+1000)%nums.len()]], nums[output[(zero+2000)%nums.len()]], nums[output[(zero+3000)%nums.len()]]);
    println!("{},{},{} : {}", coords.0,coords.1,coords.2, coords.0+coords.1+coords.2);
}
