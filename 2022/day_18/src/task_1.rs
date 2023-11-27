use std::collections::HashSet;
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

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let droplets: HashSet<Coord> = contents.lines().map(|x| x.parse().unwrap()).collect();

    let mut area = 0;
    for d in droplets.clone(){
        for n in d.adjacent() {
            if !droplets.contains(&n) {
                area += 1;
            }
        }
    }

    println!("{}", area);
}
