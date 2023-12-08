
pub fn task_2() {
    let (p, v) = include_str!("../input1").split_once("\n\n").unwrap();
    let mut transforms: Vec<(usize,usize)> = vec![(0,0); 36usize.pow(4)];
    let mut pos: Vec<usize> = Vec::new();
    for line in v.lines() {
        let (i, t) = line.split_once("=").unwrap();
        if i.trim().ends_with("A") {
            pos.push(to_b36(i));
        }
        let t = t.replace("(", "").replace(")", "");
        let (l, r) = t.split_once(",").unwrap();
        transforms[to_b36(i)] = (to_b36(l), to_b36(r));
    }
    let mut i = 0;
    let p: Vec<char> = p.chars().collect();
    let mut fastest: Vec<usize> = Vec::new();
    while pos.len() > 0 {
        pos = pos.iter().map(|x| {
            if x % 36 == 35 {
                fastest.push(i);
                return None;
            }
            match p[i % p.len()] {
                'L' => Some(transforms[*x].0),
                'R' => Some(transforms[*x].1),
                _ => panic!("Invalid input"),
            }
        }).filter(|x| x.is_some()).map(|x| x.unwrap()).collect();

        i += 1;    
        println!("{}", i);
    }

    let gcm = fastest.iter().fold(1, |acc, x| num::integer::lcm(acc, *x));

    println!("Task 2: {}", gcm);
}   

fn to_b36(s: &str) -> usize {
    s.trim().chars().map(|c| c.to_digit(36).unwrap() as usize).rev().enumerate().fold(0, |acc, (i, x)| acc + x * 36usize.pow(i as u32))
}
