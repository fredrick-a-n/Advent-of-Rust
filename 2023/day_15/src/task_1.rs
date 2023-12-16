pub fn task_1() {
    println!(
        "Task 1: {}",
        include_str!("../input1").split(',').fold(0, |acc, x| acc
            + x.chars().fold(0, |acc, x| ((acc + x as u32) * 17) % 256))
    )
}
