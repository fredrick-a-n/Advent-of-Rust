use std::collections::LinkedList;
use std::fs;
use std::path::Path;


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    let mut start: Vec<(usize,usize)> = Vec::new();
    let mut end: (usize,usize) = (0,0);
    for l in contents.lines(){
        height_map.push(Vec::new());
        for c in l.to_string().chars(){
            if c == 'a' || c == 'S'{
                start.push((height_map.len()-1, height_map.last().unwrap().len()));
                height_map.last_mut().unwrap().push('a'.to_digit(36).unwrap() as i32);
            } else if c == 'E' {
                end = (height_map.len()-1, height_map.last().unwrap().len());
                height_map.last_mut().unwrap().push('z'.to_digit(36).unwrap() as i32);
            } else {
                height_map.last_mut().unwrap().push(c.to_digit(36).unwrap() as i32);
            }
        }
    }
    let mut results: Vec<i32> = Vec::new();
    let max_y = height_map.get(0).unwrap().len() as i32;
    let max_x = height_map.len() as i32;
    for s in start {
        let mut quickest_path = vec![vec![i32::max_value(); max_y as usize];  max_x as usize];
        let mut explore: LinkedList<(usize,usize)> = LinkedList::new();
        explore.push_back(s);
        quickest_path[s.0][s.1] = 0;

        


        while explore.len() > 0 {
            let c = explore.pop_front().unwrap();
            if c == end {
                results.push(quickest_path.get(c.0).unwrap().get(c.1).unwrap().clone());
                break;
            }
            let c_height = height_map.get(c.0).unwrap().get(c.1).unwrap().clone();

            if c.0 as i32 + 1 < max_x {
                if (height_map.get(c.0 + 1).unwrap().get(c.1).unwrap() - c_height) <= 1 
                && quickest_path.get(c.0 + 1).unwrap().get(c.1).unwrap().clone()
                > quickest_path.get(c.0).unwrap().get(c.1).unwrap() + 1 {
                    quickest_path[c.0 + 1][c.1] = quickest_path[c.0][c.1].clone() + 1;
                    explore.push_back((c.0 + 1, c.1));
                }
            }
            if (c.0 as i32 - 1) >= 0 {
                if (height_map.get(c.0 - 1).unwrap().get(c.1).unwrap() - c_height) <= 1 
                && quickest_path.get(c.0 - 1).unwrap().get(c.1).unwrap().clone() 
                > quickest_path.get(c.0).unwrap().get(c.1).unwrap() + 1 {
                    quickest_path[c.0 - 1][c.1] = quickest_path[c.0][c.1].clone() + 1;
                    explore.push_back((c.0 - 1, c.1));
                }
            }
            if c.1 as i32 + 1 < max_y {
                if (height_map.get(c.0).unwrap().get(c.1 + 1).unwrap() - c_height) <= 1 
                && quickest_path.get(c.0).unwrap().get(c.1 + 1).unwrap().clone() 
                > quickest_path.get(c.0).unwrap().get(c.1).unwrap() + 1 {
                    quickest_path[c.0][c.1 + 1] = quickest_path[c.0][c.1].clone() + 1;
                    explore.push_back((c.0, c.1 + 1));
                }
            }
            if c.1 as i32 - 1 >= 0 {
                if (height_map.get(c.0).unwrap().get(c.1 - 1).unwrap() - c_height) <= 1 
                && quickest_path.get(c.0).unwrap().get(c.1 - 1).unwrap().clone() 
                > quickest_path.get(c.0).unwrap().get(c.1).unwrap() + 1{
                    quickest_path[c.0][c.1 - 1] = quickest_path[c.0][c.1].clone() + 1;
                    explore.push_back((c.0, c.1 - 1));
                }
            }
        }
    }
    results.sort();

    let out = results.first().unwrap();

    println!("{}",out);
}