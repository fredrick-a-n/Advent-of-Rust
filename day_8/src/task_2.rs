use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn value(&self) -> (i32,i32) {
        match *self {
            Direction::North => (0,-1),
            Direction::South => (0,1),
            Direction::West => (-1,0),
            Direction::East => (1,0),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Tree{
    height: i32,
    visible: bool,
    coords: (usize,usize),
}

impl Tree{
    pub fn new(height: i32, coords: (usize,usize)) -> Self {
        Self {
            height,
            visible: false,
            coords
        }
    }

}


fn initiate_matrix(n: i32, contents: String) -> Vec<Vec<Tree>> {
    let mut matrix:Vec<Vec<Tree>> = vec![vec![]; n as usize];
    let s = contents.split("\n");
    let mut i = 0;
    for row in s {
        let mut j = 0;
        for c in row.chars() {
            matrix.get_mut(j).unwrap().insert(i, Tree::new(c.to_digit(10).unwrap() as i32, (j, i)));
            j += 1;
        }
        i += 1;
    }
    return matrix;
}

pub fn visibility(x: i32, y: i32, trees: &mut Vec<Vec<Tree>>, dir: Direction, own_height: i32) -> i32{
    let new_x = x + dir.value().0;
    let new_y = y + dir.value().1;
    if new_x >= trees.len() as i32 || new_y >= trees.len() as i32|| new_x < 0 || new_y < 0{
        return 0;
    }
    let t = trees.get(new_x as usize).unwrap().get(new_y as usize).unwrap().height;
    if t >= own_height {
        return 1;
    }
    return 1 + visibility(new_x, new_y, trees, dir, own_height);
}

pub fn task_2(){
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let n = f64::sqrt(contents.len() as f64) as i32;
    let mut trees = initiate_matrix(n, contents);
    let mut most_scenic = 0;
    for i in 0..n {
        for j in 0..n {
            let mut scenic_score = 1;
            let own_height = trees.get(i as usize).unwrap().get(j as usize).unwrap().height;
            scenic_score *= visibility(i, j, &mut trees, Direction::East, own_height);
            scenic_score *= visibility(i, j, &mut trees, Direction::South, own_height);
            scenic_score *= visibility(i, j, &mut trees, Direction::West, own_height);
            scenic_score *= visibility(i, j, &mut trees, Direction::North, own_height);
            if scenic_score > most_scenic {
                most_scenic = scenic_score;
            }
        }
    }
    println!("{}", most_scenic);
}