pub fn task_2() {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for lens in include_str!("../testinput").split(",") {
        if lens.contains("=") {
            let (m, i) = lens.split_once("=").unwrap();
            let i = i.parse::<usize>().unwrap();
            let mj = m.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256);
            if let Some(j) = boxes[mj].iter().position(|x| x.0 == m) {
                boxes[mj][j].1 = i;
            } else {
                boxes[mj].push((m, i));
            }
        } else {
            let m = lens.split_once("-").unwrap().0;
            let mj = m.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256);
            if let Some(j) = boxes[mj].iter().position(|x| x.0 == m) {
                boxes[mj].remove(j);
            }
        }
    }
    let score = boxes
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(|(j, x)| (i + 1) * (j + 1) * x.1)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Task 2: {}", score)
}
