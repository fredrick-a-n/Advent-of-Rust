mod task_1;
mod task_2;

fn main() {
    let now: std::time::Instant = std::time::Instant::now();
    task_1::task_1();
    task_2::task_2();
    println!("Took {}ms", now.elapsed().as_millis());
}
