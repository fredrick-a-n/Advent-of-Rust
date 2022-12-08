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

pub fn make_visible(x: i32, y: i32, trees: &mut Vec<Vec<Tree>>, dir: Direction, last: i32){
    let mut highest = last; 
    let t = trees.get(x as usize).unwrap().get(y as usize).unwrap().height;
    if t > last {
        highest = t;
        trees.get_mut(x as usize).unwrap().get_mut(y as usize).unwrap().visible = true;
    } 
    let new_x = x + dir.value().0;
    let new_y = y + dir.value().1;
    if new_x >= trees.len() as i32 || new_y >= trees.len() as i32|| new_x < 0 || new_y < 0{
        return;
    }
    make_visible(new_x, new_y, trees, dir, highest)
}

pub fn task_1(){
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let n = f64::sqrt(contents.len() as f64) as i32;
    let mut trees = initiate_matrix(n, contents);
    for i in 0..n{
        make_visible(0, i, &mut trees, Direction::East, -1);
        make_visible(i, 0, &mut trees, Direction::South, -1);
        make_visible(n-1, i, &mut trees, Direction::West, -1);
        make_visible(i, n-1, &mut trees, Direction::North, -1);
    }

    //let temp = trees.iter().map(|x| x.iter().map(|y| (y.coords,y.height,y.visible)));
    //dbg!(temp);
    let ans = trees.iter().map(|x| x.iter().map(|y| y.visible)).flatten().filter(|x| *x).count();
    println!("{}", ans);
}