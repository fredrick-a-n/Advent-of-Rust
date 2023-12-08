use rayon::prelude::*;

pub fn task_2() {
    let (p, v) = include_str!("../input1").split_once("\n\n").unwrap();
    let mut transforms: Vec<(usize,usize)> = vec![(0,0); 36usize.pow(4)];
    let mut pos: Vec<usize> = Vec::new();
    for line in v.lines() {
        let (i, t) = line.split_once("=").unwrap();
        let j: usize = to_b36(i);
        if i.trim().ends_with("A") {
            pos.push(j);
        }
        let t = t.replace("(", "").replace(")", "");
        let (l, r) = t.split_once(",").unwrap();
        transforms[j] = (to_b36(l), to_b36(r));
    }
    let mut i = 0;
    let p: Vec<char> = p.chars().collect();
    let ll = p.len();
    let mut j = 0;
    while pos.iter().any(|&x| x % 36 != 35) {
        pos = pos.into_par_iter().map(|x| {
            match p[i] {
                'L' => transforms[x].0,
                'R' => transforms[x].1,
                _ => panic!("Invalid input"),
            }
        }).collect();
        i += 1;    
        if i >= ll {
            i = 0;
            j += i;
        }
        println!("{}", i+j*ll);
    }

    println!("Task 2: {}", (i+j*ll));
}   

fn to_b36(s: &str) -> usize {
    s.trim().chars().map(|c| c.to_digit(36).unwrap() as usize).rev().enumerate().fold(0, |acc, (i, x)| acc + x * 36usize.pow(i as u32))
}
