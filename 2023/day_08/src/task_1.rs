pub fn task_1() {
    let (p, v) = include_str!("../input1").split_once("\n\n").unwrap();
    let mut transforms: Vec<(usize,usize)> = vec![(0,0); 36usize.pow(4)];
    for line in v.lines() {
        let (i, t) = line.split_once("=").unwrap();
        let i: usize = to_b36(i);
        
        let t = t.replace("(", "").replace(")", "");
        let (l, r) = t.split_once(",").unwrap();
        transforms[i] = (to_b36(l), to_b36(r));
    }
    let mut i = 0;
    let mut pos = to_b36("AAA");
    let p: Vec<char> = p.chars().collect();
    while pos != to_b36("ZZZ") {
        match p[i % p.len()] {
            'L' => pos = transforms[pos].0,
            'R' => pos = transforms[pos].1,
            _ => panic!("Invalid input"),
        }
        i += 1;    
    }
    println!("Task 1: {}", i);
}   

fn to_b36(s: &str) -> usize {
    s.trim().chars().map(|c| c.to_digit(36).unwrap() as usize).rev().enumerate().fold(0, |acc, (i, x)| acc + x * 36usize.pow(i as u32))
}
