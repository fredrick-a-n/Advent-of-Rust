use std::collections::HashMap;

pub fn task_2() {
    let mut grid = include_str!("../input1")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut oo = 0;
    let mut cycle: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let stone_count = grid
        .iter()
        .map(|r| r.iter().filter(|c| **c == 'O').count())
        .sum::<usize>();
    loop {
        cycle.insert(grid.clone(), oo);
        for _ in 0..4 {
            for k in (0..=grid.len()).rev() {
                let mut count = 0;
                for i in 0..k {
                    for j in 0..grid[0].len() {
                        if grid[i][j] == 'O' {
                            if i != 0 && grid[i - 1][j] == '.' {
                                grid[i - 1][j] = 'O';
                                grid[i][j] = '.';
                            } else {
                                count += 1;
                            }
                        }
                    }
                }
                if count == stone_count {
                    break;
                }
            }
            grid = rotate(grid);
        }
        oo += 1;
        println!("Round {}", oo);
        if cycle.contains_key(&grid) {
            break;
        }
    }

    let mm = cycle[&grid];
    let c = (1000000000 - oo) % (oo - mm);
    let score: usize = cycle
        .iter()
        .find(|(_, i)| **i == mm + c)
        .unwrap()
        .0
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .filter(|y| **y == 'O')
                .map(|_| grid.len() - i)
                .sum::<usize>()
        })
        .sum();
    println!("Task 2: {}", score);
}

fn rotate(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}