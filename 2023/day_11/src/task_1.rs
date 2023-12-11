pub fn task_1() {
    let mut universe = include_str!("../input1")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for i in (0..universe.len()).rev() {
        if universe[i].iter().all(|c| *c == '.') {
            universe.insert(i, universe[i].clone());
        }
    }
    for i in (0..universe[0].len()).rev() {
        if universe.iter().all(|v| v[i] == '.') {
            for j in 0..universe.len() {
                universe[j].insert(i, '.');
            }
        }
    }
    let galaxies: Vec<(usize, usize)> = universe
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();
    let score: i128 = (0..galaxies.len())
        .map(|i| {
            (i + 1..galaxies.len())
                .map(|j| {
                    (galaxies[i].0 as i128 - galaxies[j].0 as i128).abs()
                        + (galaxies[i].1 as i128 - galaxies[j].1 as i128).abs()
                })
                .sum::<i128>()
        })
        .sum();
    println!("Task 1: {}", score)
}
