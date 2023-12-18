use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn task_2() {
    let grid: Vec<Vec<i64>> = include_str!("../input1")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect();
    println!("Task 2: {}", dijkstra(&grid));
}

fn dijkstra(grid: &Vec<Vec<i64>>) -> i64 {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, (0, 0), (1, 0))));
    queue.push(Reverse((0, (0, 0), (0, 1))));
    while let Some(Reverse((dist, pos, dir))) = queue.pop() {
        if pos == (grid.len() as i32 - 1, grid[0].len() as i32 - 1) {
            return dist;
        }
        if let Some(d) = dists.get(&(pos, dir)) {
            if *d > dist {
                continue;
            }
        }
        for n_dir in [(dir.1, dir.0), (-dir.1, -dir.0)] {
            let mut n_dist = dist;
            for d in 1..11 {
                let n_pos = (pos.0 + n_dir.0 * d, pos.1 + n_dir.1 * d);
                if n_pos.0 >= grid.len() as _
                    || n_pos.1 >= grid[0].len() as _
                    || n_pos.0 < 0
                    || n_pos.1 < 0
                {
                    break;
                }
                n_dist += grid[n_pos.0 as usize][n_pos.1 as usize];
                if d >= 4 && n_dist < *dists.get(&(n_pos, n_dir)).unwrap_or(&i64::MAX) {
                    dists.insert((n_pos, n_dir), n_dist);
                    queue.push(Reverse((n_dist, n_pos, n_dir)));
                }
            }
        }
    }
    unreachable!()
}
