use std::collections::HashSet;
use std::fs;
use std::path::Path;


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();

    let mut blizzards = contents.lines()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate()
        .filter(|(_,c)| *c == '<' || *c == '>' || *c == '^' || *c == 'v')
        .map(move |(x,c)| (x,y,c))).collect::<Vec<(usize, usize, char)>>();

    
    let mut positions: HashSet<(usize,usize)> = HashSet::new();
    positions.insert((1,0));
    let mut moves = 0;
    'outer1: loop {
        let mut will_break = false;
        for (x,y) in positions.clone() {
            if x == cols-2 && y == rows-2 {
                will_break = true;
            }
            if x > 1 && y != 0 {
                positions.insert((x-1, y));
            }
            if x < cols-2 && y != 0{
                positions.insert((x+1, y));
            }
            if y > 1 {
                positions.insert((x, y-1));
            }
            if y < rows-2 {
                positions.insert((x, y+1));
            }
        }  

        blizzards = blizzards.iter().map(|(x,y,dir)| match dir {
            '>' => if *x >= cols-2 {
                    (1,*y,*dir)
                } else {
                    (x+1,*y,*dir)
                },
            'v' => if *y >= rows-2 {
                    (*x,1,*dir)
                } else {
                    (*x,y+1,*dir)
                },
            '<' => if *x <= 1 {
                    (cols-2,*y,*dir)
                } else {
                    (x-1,*y,*dir)
                },
            '^' => if *y <= 1 {
                    (*x,rows-2,*dir)
                } else {
                    (*x,y-1,*dir)
                },
            _ => unreachable!()
        }).collect();
        for (x,y,_) in blizzards.clone() {
            positions.remove(&(x,y));
        }
        moves += 1;
        if will_break {
            break 'outer1;
        }
        //println!("next {} {} {}", moves, positions.len(), blizzards.len());
    }
    positions.clear();
    positions.insert((cols-2,rows-1));
    'outer2: loop {
        let mut will_break = false;

        for (x,y) in positions.clone() {
            if x == 1 && y == 1 {
                will_break = true;
            }
            if x > 1 && y != rows-1 {
                positions.insert((x-1, y));
            }
            if x < cols-2 && y != rows-1{
                positions.insert((x+1, y));
            }
            if y > 1 {
                positions.insert((x, y-1));
            }
            if y < rows-2 {
                positions.insert((x, y+1));
            }
        }  

        blizzards = blizzards.iter().map(|(x,y,dir)| match dir {
            '>' => if *x >= cols-2 {
                    (1,*y,*dir)
                } else {
                    (x+1,*y,*dir)
                },
            'v' => if *y >= rows-2 {
                    (*x,1,*dir)
                } else {
                    (*x,y+1,*dir)
                },
            '<' => if *x <= 1 {
                    (cols-2,*y,*dir)
                } else {
                    (x-1,*y,*dir)
                },
            '^' => if *y <= 1 {
                    (*x,rows-2,*dir)
                } else {
                    (*x,y-1,*dir)
                },
            _ => unreachable!()
        }).collect();
        for (x,y,_) in blizzards.clone() {
            positions.remove(&(x,y));
        }
        moves += 1;
        if will_break {
            break 'outer2;
        }
        //println!("next {} {} {}", moves, positions.len(), blizzards.len());
    }
    positions.clear();
    positions.insert((1,0));
    'outer3: loop {
        let mut will_break = false;
        for (x,y) in positions.clone() {
            if x == cols-2 && y == rows-2 {
                will_break = true;
            }
            if x > 1 && y != 0 {
                positions.insert((x-1, y));
            }
            if x < cols-2 && y != 0{
                positions.insert((x+1, y));
            }
            if y > 1 {
                positions.insert((x, y-1));
            }
            if y < rows-2 {
                positions.insert((x, y+1));
            }
        }  

        blizzards = blizzards.iter().map(|(x,y,dir)| match dir {
            '>' => if *x >= cols-2 {
                    (1,*y,*dir)
                } else {
                    (x+1,*y,*dir)
                },
            'v' => if *y >= rows-2 {
                    (*x,1,*dir)
                } else {
                    (*x,y+1,*dir)
                },
            '<' => if *x <= 1 {
                    (cols-2,*y,*dir)
                } else {
                    (x-1,*y,*dir)
                },
            '^' => if *y <= 1 {
                    (*x,rows-2,*dir)
                } else {
                    (*x,y-1,*dir)
                },
            _ => unreachable!()
        }).collect();
        for (x,y,_) in blizzards.clone() {
            positions.remove(&(x,y));
        }
        moves += 1;
        if will_break {
            break 'outer3;
        }
        //println!("next {} {} {}", moves, positions.len(), blizzards.len());
    }
    println!("{}", moves);

}
