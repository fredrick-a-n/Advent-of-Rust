use itertools::Itertools;
use std::collections::HashSet;

pub fn task_1() {
    let grid: Vec<Vec<char>> = include_str!("../input1")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut history: HashSet<((i8, i8), (i8, i8))> = HashSet::new();
    rec(
        &mut history,
        &grid,
        (0, 0),
        (0, 1),
        &(grid.len() as i8),
        &(grid[0].len() as i8),
    );
    println!(
        "Task 1: {}",
        history.iter().map(|(x, _)| x).unique().count()
    );
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
