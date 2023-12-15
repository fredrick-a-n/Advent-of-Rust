pub fn task_2() {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for lens in include_str!("../input1").split(",") {
        if lens.contains("=") {
            let (l, i) = lens.split_once("=").unwrap();
            let i = i.parse::<usize>().unwrap();
            let hash = l.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256);
            if let Some(j) = boxes[hash].iter().position(|x| x.0 == l) {
                boxes[hash][j].1 = i;
            } else {
                boxes[hash].push((l, i));
            }
        } else {
            let l = lens.split_once("-").unwrap().0;
            let hash = l.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256);
            if let Some(j) = boxes[hash].iter().position(|x| x.0 == l) {
                boxes[hash].remove(j);
            }
        }
    }
    println!(
        "Task 2: {}",
        boxes.iter().enumerate().fold(0, |acc, (i, x)| {
            acc + x
                .iter()
                .enumerate()
                .fold(0, |acc, (j, x)| acc + (i + 1) * (j + 1) * x.1)
        })
    )
}
