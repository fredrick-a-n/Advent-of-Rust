use std::collections::LinkedList;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Turn(char),
    Move(usize)
}

const KEK: usize = 50;

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let mut board: Vec<Vec<char>> = Vec::new();
    let mut moves: LinkedList<Move> = LinkedList::new();
    let mut k = contents.split("\n\n");
    for l in k.next().unwrap().lines(){
        board.push(l.chars().collect());
    }
    let re= r"(\d+)(R|L)?".to_owned();    
    for cap in Regex::new(&re).unwrap().captures_iter(k.last().unwrap()){ 
        moves.push_back(Move::Move(cap[1].parse().unwrap()));
        if cap.get(2).is_some() {
            moves.push_back(Move::Turn(cap[2].parse().unwrap()));
        }
    }
    let mut position = (board[0].iter().position(|x| *x == '.').unwrap(), 0);
    let mut dir: (i32,i32) = (1,0);
    for m in moves {
        match m {
            Move::Move(n) => for _ in 0..n {
                println!("MOVE {} ({},{})", n, position.0, position.1);
                let new_pos = (position.0 as i32 + dir.0, position.1 as i32 + dir.1);
                if board.get(new_pos.1 as usize).is_none()
                || board[new_pos.1 as usize].get(new_pos.0 as usize).is_none()
                || board[new_pos.1 as usize][new_pos.0 as usize] == ' '{
                    println!("{}    {}", dir.0, dir.1);
                    let (qr, qc, ndir) = match (position.1 / KEK, position.0 / KEK, dir) {
                        (0,1,(-1,0)) => (2,0,(0,1)),
                        (0,1,(0,-1)) => (3,0,(1,0)),
                        (0,2,(1,0)) => (2,1,(-1,0)),
                        (0,2,(0,1)) => (1,1,(-1,0)),
                        (0,2,(0,-1)) => (3,0,(0,-1)),
                        (1,1,(1,0)) => (0,2,(0,1)),
                        (1,1,(-1,0)) => (2,0,(0,-1)),
                        (2,0,(-1,0)) => (0,1,(1,0)),
                        (2,0,(0,-1)) => (1,1,(1,0)),
                        (2,1,(1,0)) => (0,2,(-1,0)),
                        (2,1,(0,1)) => (3,0,(-1,0)),
                        (3,0,(1,0)) => (2,1,(0,1)),
                        (3,0,(-1,0)) => (0,1,(0,1)),
                        (3,0,(0,1)) => (0,2,(0,1)),
                        _ => unreachable!(),
                    };
                    let (dr, dc) = (position.1 % KEK, position.0 % KEK);
                    let i = [dr, dc, KEK-1-dr, KEK-1-dc][match dir {
                        (1,0) => 0,
                        (0,1) => 1,
                        (-1,0) => 2,
                        (0,-1) => 3,
                        _ => unreachable!()
                    }];
                    let (nr, nc) = [(KEK-1,i), (i,0), (0,KEK-1-i), (KEK-1-i,KEK-1)][match dir {
                        (1,0) => 0,
                        (0,1) => 1,
                        (-1,0) => 2,
                        (0,-1) => 3,
                        _ => unreachable!()
                    }];
                    if board[qr * KEK + nr][qc * KEK + nc] != '#' {
                        dir = ndir;
                        position = ( qc * KEK + nc, qr * KEK + nr)
                    } else {
                        break;
                    }
                } else {
                    if board[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                        position = (new_pos.0 as usize, new_pos.1 as usize);
                    } else {
                        break;
                    }
                }
            },
            Move::Turn(c) => {
                match c {
                    'R' => {println!("TURN R"); dir = (-dir.1, dir.0)},
                    'L' => {println!("TURN L"); dir = (dir.1, -dir.0)},
                    _ => ()
                }
            }
        }
    }
    let mut dir_num = 0;
    match dir {
        (1,0) => dir_num = 0,
        (0,1) => dir_num = 1,
        (-1,0) => dir_num = 2,
        (0,-1) => dir_num = 3,
        _ => ()
    }

    let password = (position.1+1) * 1000 + (position.0+1) * 4 + dir_num;

    println! ("({},{})    {}", position.0+1, position.1+1, password);
}
