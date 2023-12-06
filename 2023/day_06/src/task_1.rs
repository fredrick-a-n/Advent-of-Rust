use std::fs;
use std::path::Path;

pub fn task_1() {
    let data: Vec<Vec<i32>> = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file")
        .split("\n").map(|s| s.trim().split(" ").filter(|x| !x.trim().is_empty()).skip(1).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let data: Vec<(i32,i32)> = data.first().unwrap().iter().zip(data.last().unwrap().iter()).map(|(a, b)| (*a, *b)).collect();

    let ans = data.iter().map(|(a, b)| {
        let a = *a as f32;
        let b = *b as f32;
        let mut t1 = a/2.0 - (a*a/4.0 - b).sqrt();
        let t2 = a/2.0 + (a*a/4.0 - b).sqrt();
        t1 = if t1.fract() == 0.0 {t1+1.0} else {t1};
        (t1.ceil(), t2.ceil())
        })
        .map(|(t1, t2)| {
            (t2-t1) as i32
        })
        .reduce(|a, b| a*b).unwrap();
    println!("{}", ans);
    // x = time waited, t = total time, y = distance
    // y = (t-x)*x
    // y = tx - x^2
    // 0 = x^2 - tx + y
    // x = t/2 ± sqrt(t^2/4 - y)
}   

