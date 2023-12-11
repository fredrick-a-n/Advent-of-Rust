pub fn task_2() {
    let universe = include_str!("../input1")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let empty_rows: Vec<usize> = universe
        .iter()
        .enumerate()
        .filter(|(_, v)| v.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let empty_cols: Vec<usize> = (0..universe[0].len())
        .filter(|&i| universe.iter().all(|v| v[i] == '.'))
        .collect::<Vec<usize>>();
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
                        + empty_rows
                            .iter()
                            .filter(|&&r| {
                                galaxies[i].0.max(galaxies[j].0) > r
                                    && galaxies[i].0.min(galaxies[j].0) < r
                            })
                            .map(|_| 999999)
                            .sum::<i128>()
                        + empty_cols
                            .iter()
                            .filter(|&&c| {
                                galaxies[i].1.max(galaxies[j].1) > c
                                    && galaxies[i].1.min(galaxies[j].1) < c
                            })
                            .map(|_| 999999)
                            .sum::<i128>()
                })
                .sum::<i128>()
        })
        .sum();
    println!("Task 2: {}", score)
}
