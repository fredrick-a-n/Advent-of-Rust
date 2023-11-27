use std::collections::{HashSet, LinkedList};
use std::fs;
use std::path::Path;
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord{
    x: i64,
    y: i64,
    z: i64,
}

impl Coord{
    fn new(x: i64, y: i64, z: i64) -> Coord {
        Coord { x, y, z }
    }

    fn adjacent(&self) -> [Coord; 6] {
        [
            Coord::new(self.x + 1, self.y, self.z),
            Coord::new(self.x - 1, self.y, self.z),
            Coord::new(self.x, self.y + 1, self.z),
            Coord::new(self.x, self.y - 1, self.z),
            Coord::new(self.x, self.y, self.z + 1),
            Coord::new(self.x, self.y, self.z - 1),            
        ]
    }

}

impl FromStr for Coord {
    type Err = ();    

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<&str> = s.split(',').collect();
        Ok(Coord {
            x: c[0].parse().unwrap(),
            y: c[1].parse().unwrap(),
            z: c[2].parse().unwrap(),
        })
    }
}

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let mut droplets: HashSet<Coord> = contents.lines().map(|x| x.parse().unwrap()).collect();

    let mut init_area = 0;
    for d in droplets.clone(){
        for n in d.adjacent() {
            if !droplets.contains(&n) {
                init_area += 1;
            }
        }
    }

    let min_x = droplets.iter().map(|x| x.x).min().unwrap() - 2;
    let max_x = droplets.iter().map(|x| x.x).max().unwrap() + 2;
    let min_y = droplets.iter().map(|x| x.y).min().unwrap() - 2;
    let max_y = droplets.iter().map(|x| x.y).max().unwrap() + 2;
    let min_z = droplets.iter().map(|x| x.z).min().unwrap() - 2;
    let max_z = droplets.iter().map(|x| x.z).max().unwrap() + 2;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            droplets.insert(Coord::new(x, y, min_z));
            droplets.insert(Coord::new(x, y, max_z));
        }
    }
    for x in min_x..=max_x {
        for z in min_z..=max_z {
            droplets.insert(Coord::new(x, min_y, z));
            droplets.insert(Coord::new(x, max_y, z));
        }
    }
    for y in min_y..=max_y {
        for z in min_z..=max_z {
            droplets.insert(Coord::new(min_x, y, z));
            droplets.insert(Coord::new(max_x, y, z));
        }
    }

    let mut to_fill: LinkedList<Coord> = LinkedList::new();
    to_fill.push_back(Coord::new(min_x+1, min_y+1, min_z+1));
    droplets.insert(Coord::new(min_x+1, min_y+1, min_z+1));

    while !to_fill.is_empty() {
        let p = to_fill.pop_front().unwrap();
        for n in p.adjacent() {
            if !droplets.contains(&n) {
                droplets.insert(n);
                to_fill.push_back(n);
            }
        }
    }

    let mut total_area = 0;
    for d in droplets.clone(){
        for n in d.adjacent() {
            if !droplets.contains(&n) {
                total_area += 1;
            }
        }
    }

    let surface = 2 * (max_x - min_x + 1) * (max_y - min_y + 1)
    + 2 * (max_x - min_x + 1) * (max_z - min_z + 1)
    + 2 * (max_y - min_y + 1) * (max_z - min_z + 1);

    println!("{}", init_area-total_area+surface);


}
