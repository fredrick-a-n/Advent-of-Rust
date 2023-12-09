pub fn task_2() {
    println!("Task 2 {}",
    include_str!("../input1").lines()
        .map(|line| line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .map(|line|
            line.iter().rev().map(|x| *x).collect::<Vec<i32>>()
        )
        .map(|line| {
            std::iter::successors(Some((line, 0)), |(l, _)| 
                if l.iter().any(|x| *x != 0) {
                    Some((l.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>(), *l.last().unwrap()))
                } else {
                    None
                }
            ).fold(0, |acc, (_, n)| acc+n )
        })
        .sum::<i32>()
    )
}