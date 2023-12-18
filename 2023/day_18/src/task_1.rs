pub fn task_1() {
    let steps: Vec<Vec<&str>> = include_str!("../input1")
        .lines()
        .map(|l| l.split(' ').collect())
        .collect();
    let mut pos = (0, 0);
    let mut area = 1.0;
    for step in steps {
        let n_pos = match (step[0], step[1]) {
            ("R", n) => (pos.0, pos.1 + n.parse::<i32>().unwrap() * 2),
            ("L", n) => (pos.0, pos.1 - n.parse::<i32>().unwrap() * 2),
            ("U", n) => (pos.0 + n.parse::<i32>().unwrap() * 2, pos.1),
            ("D", n) => (pos.0 - n.parse::<i32>().unwrap() * 2, pos.1),
            _ => panic!("Invalid input"),
        };
        area += (pos.0 * n_pos.1 - pos.1 * n_pos.0) as f64 / 8.0
            + ((pos.0 - n_pos.0).abs() + (pos.1 - n_pos.1).abs()) as f64 / 4.0;
        pos = n_pos;
    }
    println!("Task 1: {}", area);
}
