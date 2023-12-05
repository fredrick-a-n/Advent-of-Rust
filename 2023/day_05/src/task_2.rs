use std::fs;
use std::path::Path;

pub fn task_2() {
    let data: Vec<String> = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file")
        .split("\n\n").map(|x| x.to_string()).collect();
    let mut transforms: Vec<Vec<(i128,i128,i128)>> = Vec::new();
    let nums: Vec<i128> = data[0].split(" ").filter(|x| x.parse::<i128>().is_ok()).map(|x| x.parse::<i128>().unwrap()).collect();
    let mut ranges: Vec<(i128,i128)> = nums.iter().zip(nums.iter().skip(1)).map(|(a,b)| (*a,*a+*b)).step_by(2).collect();
   
    for i in 1..data.len() {
        let mut transform: Vec<(i128, i128, i128)> = Vec::new();
        for line in data[i].split("\n").skip(1) {
            let mut line = line.split(" ");
            let to = line.next().unwrap().parse::<i128>().unwrap();
            let from = line.next().unwrap().parse::<i128>().unwrap();
            let range = line.next().unwrap().parse::<i128>().unwrap();
            transform.push((to, from, range));
        }
        transforms.push(transform);
    }
    for transform in transforms {
        ranges = transform_ranges(ranges, transform);
    }

    let num = ranges.iter().fold(i128::MAX,|acc, (a,_)| if acc < *a {acc} else {*a} );
    println!("Task 2: {}", num);
} 

fn transform_ranges(ranges:Vec<(i128,i128)>, transform: Vec<(i128,i128,i128)>) -> Vec<(i128,i128)>{
    if transform.len() == 0 {
        return ranges;
    }
    let mut new_ranges: Vec<(i128,i128)> = Vec::new();
    let mut old_ranges: Vec<(i128,i128)> = Vec::new();
    for num_range in ranges.iter() {
        let (to, from, range) = transform.first().unwrap();
        let (to, from , range) = (*to, *from, *range);
        match ( num_range.0 >= from, 
                num_range.0 > from+range, 
                num_range.1 >= from, 
                num_range.1 > from+range) {
            (true,false,true,false) => {
                new_ranges.push((to + (num_range.0 - from), to + (num_range.1 - from)));
            },
            (true,false,true,true) => {
                new_ranges.push((to + (num_range.0 - from), to + range));
                old_ranges.push((from+range, num_range.1));
            },
            (false,false,true,false) => {
                old_ranges.push((num_range.0, from));
                new_ranges.push((to, to + (num_range.1 - from)));
            },
            (false,false,true,true) => {
                old_ranges.push((num_range.0, from));
                new_ranges.push((to, to + range));
                old_ranges.push((from+range, num_range.1));
            },
            _ => {
                old_ranges.push(num_range.clone());
            }
        }
    }
    if old_ranges.len() > 0 {
        new_ranges.append(& mut transform_ranges(old_ranges, transform[1..].to_vec()));
    }
    return new_ranges;
}