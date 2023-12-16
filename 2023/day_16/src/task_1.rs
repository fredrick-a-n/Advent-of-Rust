use std::collections::HashSet;

pub fn task_1() {
    let grid: Vec<Vec<char>> = include_str!("../input1")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut energized: HashSet<(i8, i8)> = HashSet::new();
    let mut history: HashSet<((i8, i8), (i8, i8))> = HashSet::new();
    rec(&mut energized, &mut history, &grid, (0, 0), (0, 1));
    println!("Task 1: {}", energized.len());
}

fn rec(
    energized: &mut HashSet<(i8, i8)>,
    history: &mut HashSet<((i8, i8), (i8, i8))>,
    grid: &Vec<Vec<char>>,
    pos: (i8, i8),
    dir: (i8, i8),
) {
    if pos.0 < 0
        || pos.0 >= grid.len() as i8
        || pos.1 < 0
        || pos.1 >= grid[0].len() as i8
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
        ),
        ('\\', _) => rec(
            energized,
            history,
            grid,
            (pos.0 + dir.1, pos.1 + dir.0),
            (dir.1, dir.0),
        ),
        ('-', (_, 0)) => {
            rec(energized, history, grid, (pos.0, pos.1 + 1), (0, 1));
            rec(energized, history, grid, (pos.0, pos.1 - 1), (0, -1))
        }
        ('|', (0, _)) => {
            rec(energized, history, grid, (pos.0 + 1, pos.1), (1, 0));
            rec(energized, history, grid, (pos.0 - 1, pos.1), (-1, 0))
        }
        _ => rec(
            energized,
            history,
            grid,
            (pos.0 + dir.0, pos.1 + dir.1),
            dir,
        ),
    }
}
