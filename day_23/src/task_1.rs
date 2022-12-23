use std::collections::{HashSet, HashMap};
use std::fs;
use std::path::Path;


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut elves: Vec<(i32,i32)> = Vec::new();

    for (y, l) in contents.lines().enumerate(){
        for (x, c) in l.chars().enumerate(){
            if c == '#' {
                elves.push((x as i32, y as i32));
            }
        }
    }

    for turn in 0..10 {
        let mut proposals: HashMap<(i32,i32),i32> = HashMap::new();
        let mut irr_props: HashSet<(i32,i32)> = HashSet::new();
        for (n, e) in elves.iter().enumerate() {
            if ![(-1,-1), (0,-1), (1,-1),(-1,1), (0,1), (1,1),
            (-1,-1), (-1,0), (-1,1), (1,-1), (1,0), (1,1)].iter().any(|x| elves.contains(&(e.0+x.0, e.1+x.1))){
                continue;
            }

            let directions = match turn % 4 {
                    0 => [  [(-1,-1), (0,-1), (1,-1)],                 
                            [(-1,1), (0,1), (1,1)],
                            [(-1,-1), (-1,0), (-1,1)],
                            [(1,-1), (1,0), (1,1)]],
                    1 => [  [(-1,1), (0,1), (1,1)],
                            [(-1,-1), (-1,0), (-1,1)],
                            [(1,-1), (1,0), (1,1)],                           
                            [(-1,-1), (0,-1), (1,-1)]],
                    2 => [  [(-1,-1), (-1,0), (-1,1)],
                            [(1,-1), (1,0), (1,1)],
                            [(-1,-1), (0,-1), (1,-1)],
                            [(-1,1), (0,1), (1,1)]],
                    3 => [  [(1,-1), (1,0), (1,1)],
                            [(-1,-1), (0,-1), (1,-1)],
                            [(-1,1), (0,1), (1,1)],
                            [(-1,-1), (-1,0), (-1,1)]],
                    _ => unreachable!()
                };
            for dir in directions { 
                if dir.iter().all(|x| !elves.contains(&(e.0 + x.0, e.1 + x.1))) {
                    let new_pos = (e.0 + dir[1].0, e.1 + dir[1].1);
                    if !proposals.contains_key(&new_pos) 
                    && !irr_props.contains(&new_pos){
                        proposals.insert(new_pos, n as i32);
                    } else {
                        irr_props.insert(new_pos);
                        proposals.remove(&new_pos);
                    }
                    break;
                } 
            }
        }

        for (mov, elf) in proposals.clone() {
            elves[elf as usize] = mov;
        }  
    }

    let mut max_x = i32::min_value();
    let mut min_x = i32::max_value();
    let mut max_y = i32::min_value();
    let mut min_y = i32::max_value();

    for (x,y) in elves.clone() {
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }
    }
    println!("{}", (max_x-min_x+1) * (max_y-min_y+1) - elves.len() as i32);
}

