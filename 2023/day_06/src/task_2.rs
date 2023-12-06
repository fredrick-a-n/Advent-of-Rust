use std::fs;
use std::path::Path;

pub fn task_2() {
    let binding = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let data: Vec<&str> = binding
        .split("\n").collect(); 
    let time: f64 = data.first().unwrap().trim().split_at(10).1.replace(" ", "").parse::<f64>().unwrap();
    let dist: f64 = data.last().unwrap().trim().split_at(10).1.replace(" ", "").parse::<f64>().unwrap();

    let mut t1 = time/2.0 - (time*time/4.0 - dist).sqrt();
    let t2 = time/2.0 + (time*time/4.0 - dist).sqrt();
    t1 = if t1.fract() == 0.0 {t1+1.0} else {t1};
    let ans = t2.ceil() - t1.ceil();
    println!("{}", ans);
    // t = time waited, x = total time, y = distance
    // y = (x-t)*t
    // y = xt - t^2x
    // 0 = t^2 - xt + y
    // t = x/2 ± sqrt(x^2/4 - y)
}   
