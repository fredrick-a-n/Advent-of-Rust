use std::fs;
use std::path::Path;


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./test1"))
        .expect("Should have been able to read the file");

    let mut nums: Vec<(i32, bool)> = Vec::new();
    for l in contents.lines(){
        nums.push((l.parse().unwrap(), false));
    }
    loop {
        let k = nums.iter().find(|x| x.1 == false);
        let i = nums.iter().position(|x| x.1 == false);
        if k.is_none() {
            break;
        }
        let i = i.unwrap();
        let k = k.unwrap().clone();
        if i as i32 + k.0 < 0 {
            let new_index =  (((i as i32 + k.0 -1 ) % nums.len() as i32) + nums.len() as i32) % nums.len() as i32;
            nums.remove(i);
            nums.insert(new_index as usize, (k.0.clone(), true));
        } else if i as i32 + k.0 > nums.len() as i32{
            let new_index =  (((i as i32 + k.0 +1 ) % nums.len() as i32) + nums.len() as i32) % nums.len() as i32;
            nums.remove(i);
            nums.insert(new_index as usize, (k.0.clone(), true));
        } else {
            let new_index =  (((i as i32 + k.0) % nums.len() as i32) + nums.len() as i32) % nums.len() as i32;
            nums.remove(i);
            nums.insert(new_index as usize, (k.0.clone(), true));
        }

        /*
        for l in nums.clone() {
            print!("{} ", l.0);
        }
        println!("");
        */
    }   
    let zero = nums.iter().position(|x| x.0 == 0).unwrap();

    let coords = (nums[(zero+1000)%nums.len()].0, nums[(zero+2000)%nums.len()].0, nums[(zero+3000)%nums.len()].0);
    println!("{},{},{} : {}", coords.0,coords.1,coords.2, coords.0+coords.1+coords.2);

}
