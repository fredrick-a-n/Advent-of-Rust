use rayon::iter::*;
use std::collections::HashSet;

pub fn task_2() {
    let grid: Vec<Vec<char>> = include_str!("../input1")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let k = (0..(grid.len().max(grid[0].len()) as i8))
        .into_par_iter()
        .flat_map(|i| {
            vec![
                vec![((i, 0), (0, 1)), ((i, grid[0].len() as i8 - 1), (0, -1))],
                vec![((0, i), (1, 0)), ((grid.len() as i8 - 1, i), (-1, 0))],
            ]
        })
        .flatten()
        .map(|(pos, dir)| {
            let mut energized: HashSet<(i8, i8)> = HashSet::new();
            let mut history: HashSet<((i8, i8), (i8, i8))> = HashSet::new();
            rec(&mut energized, &mut history, &grid, pos, dir, &grid.len(), &grid[0].len());
            energized.len()
        })
        .max()
        .unwrap();

    println!("Task 2: {}", k);
}

fn rec(
    energized: &mut HashSet<(i8, i8)>,
    history: &mut HashSet<((i8, i8), (i8, i8))>,
    grid: &Vec<Vec<char>>,
    pos: (i8, i8),
    dir: (i8, i8),
    l1: &usize,
    l2: &usize,
) {
    if pos.0 < 0
        || pos.0 >= *l1 as i8
        || pos.1 < 0
        || pos.1 >= *l2 as i8
        || history.contains(&(pos, dir))
    {
        return;
    }
    energized.insert(pos);
    history.insert((pos, dir));
    match (grid[pos.0 as usize][pos.1 as usize], dir) {
        ('/', _) => rec(
            energized,
            history,
            grid,
            (pos.0 - dir.1, pos.1 - dir.0),
            (-dir.1, -dir.0),
            l1,
            l2,
        ),
        ('\\', _) => rec(
            energized,
            history,
            grid,
            (pos.0 + dir.1, pos.1 + dir.0),
            (dir.1, dir.0),
            l1,
            l2,
        ),
        ('-', (_, 0)) => {
            rec(energized, history, grid, (pos.0, pos.1 + 1), (0, 1), l1, l2);
            rec(energized, history, grid, (pos.0, pos.1 - 1), (0, -1), l1, l2)
        }
        ('|', (0, _)) => {
            rec(energized, history, grid, (pos.0 + 1, pos.1), (1, 0), l1, l2);
            rec(energized, history, grid, (pos.0 - 1, pos.1), (-1, 0), l1, l2)
        }
        _ => rec(
            energized,
            history,
            grid,
            (pos.0 + dir.0, pos.1 + dir.1),
            dir,
            l1,
            l2,
        ),
    }
}
