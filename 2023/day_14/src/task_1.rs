use std::collections::HashMap;

pub fn task_1() {
    let grid = include_str!("../input1")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let round = grid
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .filter(|(_, r)| **r == 'O')
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();
    let mut cube = grid
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .filter(|(_, r)| **r == '#')
                .map(move |(j, _)| ((i, j), 0))
        })
        .collect::<HashMap<(usize, usize), usize>>();

    let mut top: Vec<usize> = vec![0; grid[0].len()];
    'outer: for (i, j) in round {
        for x in (0..i).rev() {
            if cube.contains_key(&(x, j)) {
                cube.insert((x, j), cube[&(x, j)] + 1);
                continue 'outer;
            }
        }
        top[j] += 1;
    }

    let score = cube
        .iter()
        .map(|((i, _), c)| (1..=*c).fold(0, |acc, x| acc + (grid.len() - i - x)))
        .sum::<usize>()
        + top
            .iter()
            .enumerate()
            .map(|(_, c)| (0..*c).fold(0, |acc, x| acc + (grid.len() - x)))
            .sum::<usize>();

    println!("Task 1: {}", score);
}
