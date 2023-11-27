use std::fs;
use std::path::Path;

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let mut cave = vec![vec![false;1000];1000];
    let mut max_y = 0;
    
    for l in contents.lines() {
        let mut line: Vec<(i32,i32)> = Vec::new();
        for c in l.split("->") {
            let mut n = c.split(",");
            line.push((n.next().unwrap().trim().parse().unwrap(),n.next().unwrap().trim().parse().unwrap()));
        }
        for i in 1..line.len() {
            let c1 = line.get(i-1).unwrap();
            let c2 = line.get(i).unwrap();
            for j in c1.0.min(c2.0)..(c1.0.max(c2.0)+1) {
                for k in c1.1.min(c2.1)..(c1.1.max(c2.1)+1) {
                    cave[j as usize][k as usize] = true;
                }
            }
            if c1.1 > max_y {
                max_y = c1.1;
            }
            if c2.1 > max_y {
                max_y = c2.1;
            } 
        }
    }


    
    let mut sand_tiles = 0;
    loop {
        if cave[500][0] {
            break;
        }
        let mut x:usize = 500;
        let mut y:usize = 0;
        while y <= (max_y + 1) as usize{
            if !cave[x][y+1] {
                y += 1;
            } else if !cave[x-1][y+1] {
                y += 1;
                x -= 1;
            } else if !cave[x+1][y+1] {
                y += 1;
                x += 1;
            } else {
                cave[x][y] = true;
                sand_tiles += 1;
                break;
            }
        }
        if y >= (max_y + 1) as usize {
            break;
        }
    }


    println!("{}",sand_tiles);
    
}