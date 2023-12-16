use itertools::Itertools;
use rayon::iter::*;
use std::collections::HashSet;

pub fn task_2() {
    let grid: Vec<Vec<char>> = include_str!("../input1")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let l1 = grid.len() as i8;
    let l2 = grid[0].len() as i8;
    let sum = (0..(l1.max(l2)))
        .into_par_iter()
        .flat_map(|i| {
            [
                ((i, 0), (0, 1)),
                ((i, l2 - 1), (0, -1)),
                ((0, i), (1, 0)),
                ((l1 - 1, i), (-1, 0)),
            ]
        })
        .map(|(pos, dir)| {
            let mut history: HashSet<((i8, i8), (i8, i8))> = HashSet::new();
            rec(&mut history, &grid, pos, dir, &l1, &l2);
            history.iter().map(|(x, _)| x).unique().count()
        })
        .max()
        .unwrap();
    println!("Task 2: {}", sum);
}

fn rec(
    history: &mut HashSet<((i8, i8), (i8, i8))>,
    grid: &Vec<Vec<char>>,
    pos: (i8, i8),
    dir: (i8, i8),
    l1: &i8,
    l2: &i8,
) {
    if pos.0 < 0 || pos.0 >= *l1 || pos.1 < 0 || pos.1 >= *l2 || history.contains(&(pos, dir)) {
        return;
    }
    history.insert((pos, dir));
    match (grid[pos.0 as usize][pos.1 as usize], dir) {
        ('/', _) => rec(
            history,
            grid,
            (pos.0 - dir.1, pos.1 - dir.0),
            (-dir.1, -dir.0),
            l1,
            l2,
        ),
        ('\\', _) => rec(
            history,
            grid,
            (pos.0 + dir.1, pos.1 + dir.0),
            (dir.1, dir.0),
            l1,
            l2,
        ),
        ('-', (_, 0)) => {
            rec(history, grid, (pos.0, pos.1 + 1), (0, 1), l1, l2);
            rec(history, grid, (pos.0, pos.1 - 1), (0, -1), l1, l2);
        }
        ('|', (0, _)) => {
            rec(history, grid, (pos.0 + 1, pos.1), (1, 0), l1, l2);
            rec(history, grid, (pos.0 - 1, pos.1), (-1, 0), l1, l2);
        }
        _ => rec(history, grid, (pos.0 + dir.0, pos.1 + dir.1), dir, l1, l2),
    }
}
