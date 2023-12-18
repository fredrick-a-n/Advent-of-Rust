pub fn task_2() {
    let steps: Vec<(&str, &str)> = include_str!("../input1")
        .lines()
        .map(|l| l.split_once("(#").unwrap().1.split_at(5))
        .collect();
    let mut pos = (0, 0);
    let mut area = 1.0;
    for step in steps {
        let n_pos = match (step.0, step.1) {
            (n, "0)") => (pos.0, pos.1 + i64::from_str_radix(n, 16).unwrap() * 2),
            (n, "2)") => (pos.0, pos.1 - i64::from_str_radix(n, 16).unwrap() * 2),
            (n, "3)") => (pos.0 + i64::from_str_radix(n, 16).unwrap() * 2, pos.1),
            (n, "1)") => (pos.0 - i64::from_str_radix(n, 16).unwrap() * 2, pos.1),
            _ => panic!("Invalid input"),
        };
        area += ((pos.0 * n_pos.1 - pos.1 * n_pos.0) as f64 / 2.0
            + ((pos.0 - n_pos.0).abs() + (pos.1 - n_pos.1).abs()) as f64) / 4.0;
        pos = n_pos;
    }
    println!("Task 2: {}", area);
}
