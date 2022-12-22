use std::fmt;
use std::collections::LinkedList;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Turn(char),
    Move(usize)
}

pub fn task_1() {
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
                println!("({},{})", position.0, position.1);
                let new_pos = (position.0 as i32 + dir.0, position.1 as i32 + dir.1);
                if dir.0 == 0 {
                    if new_pos.1 >= board.len() as i32 
                    || new_pos.1 >= board.len() as i32 - board.iter().rev().position(|x| x.get(new_pos.0 as usize).is_some() && *x.get(new_pos.0 as usize).unwrap() != ' ').unwrap() as i32{
                        let i = board.iter().filter(|x| x.len() as i32 >= new_pos.0).position(|x| x[new_pos.0 as usize] != ' ').unwrap();
                        if board[i][new_pos.0 as usize] != '#' {
                            position = (new_pos.0 as usize, i);
                            continue
                        } else {
                            break;
                        }
                    } else if new_pos.1 < 0
                    || new_pos.1 < board.iter().position(|x| x.get(new_pos.0 as usize).is_some() && *x.get(new_pos.0 as usize).unwrap() != ' ').unwrap() as i32 {
                        let i = board.len() - board.iter().rev().position(|x| x.get(new_pos.0 as usize).is_some() && *x.get(new_pos.0 as usize).unwrap() != ' ').unwrap() -1;
                        if board[i][new_pos.0 as usize] != '#' {
                            position = (new_pos.0 as usize, i);
                            continue
                        } else {
                            break;
                        }
                    }
                } else {
                    if new_pos.0 >= board[new_pos.1 as usize].len() as i32{
                        let i = board[new_pos.1 as usize].iter().position(|x| *x != ' ').unwrap();
                        if board[new_pos.1 as usize][i] != '#' {
                            position = (i, new_pos.1 as usize);
                            continue
                        } else {
                            break;
                        }
                    } else if new_pos.0 < board[new_pos.1 as usize].iter().position(|x| *x != ' ').unwrap() as i32 {
                        let i = board[new_pos.1 as usize].len() - 1;
                        if board[new_pos.1 as usize][i] != '#' {
                            position = (i, new_pos.1 as usize);
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                if board[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                    position = (new_pos.0 as usize, new_pos.1 as usize);
                } else {
                    break;
                }
                
            },
            Move::Turn(c) => {
                match c {
                    'R' => dir = (-dir.1, dir.0),
                    'L' => dir = (dir.1, -dir.0),
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
