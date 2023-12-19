use core::panic;
use std::collections::HashMap;

pub fn task_2() {
    let (transforms, _) = include_str!("../input1").split_once("\n\n").unwrap();
    let transforms = transforms
        .lines()
        .map(|l| l[..(l.len() - 1)].split_once("{").unwrap())
        .map(|(name, val)| {
            (
                name,
                val.split(",")
                    .map(|v| v.split_once(":").unwrap_or(("", v)))
                    .map(|(c, r)| (c.chars(), r))
                    .map(|(mut c, r)| {
                        (
                            (
                                c.next().unwrap_or('='),
                                c.next().unwrap_or('='),
                                c.as_str().parse::<i64>().unwrap_or(0),
                            ),
                            r,
                        )
                    })
                    .collect(),
            )
        })
        .collect::<HashMap<&str, Vec<((char, char, i64), &str)>>>();

    let mut accepted: Vec<HashMap<char, (i64, i64)>> = Vec::new();
    let mut ranges = vec![(
        "in",
        ['x', 'm', 'a', 's']
            .iter()
            .map(|c| (*c, (1, 4001)))
            .collect::<HashMap<char, (i64, i64)>>(),
    )];
    while let Some((key, mut input)) = ranges.pop() {
        if key == "A" {
            accepted.push(input.clone());
            continue;
        } else if key == "R" {
            continue;
        }
        let transform = transforms.get(key).unwrap();
        for t in transform {
            match *t {
                ((c, '>', v), r) => {
                    let &(low, upp) = input.get(&c).unwrap();
                    match (low > v, upp > v) {
                        (true, true) => {
                            ranges.push((r, input.clone()));
                            break;
                        }
                        (false, true) => {
                            let mut u = input.clone();
                            u.insert(c, (v + 1, upp));
                            let mut l = input.clone();
                            l.insert(c, (low, v + 1));
                            ranges.push((r, u));
                            input = l;
                        }
                        (true, false) => panic!("Invalid input"),
                        _ => continue,
                    }
                }
                ((c, '<', v), r) => {
                    let &(low, upp) = input.get(&c).unwrap();
                    match (low < v, upp < v) {
                        (true, true) => {
                            ranges.push((r, input.clone()));
                            break;
                        }
                        (true, false) => {
                            let mut u = input.clone();
                            u.insert(c, (v, upp));
                            let mut l = input.clone();
                            l.insert(c, (low, v));
                            ranges.push((r, l));
                            input = u;
                        }
                        (false, true) => panic!("Invalid input"),
                        _ => continue,
                    }
                }
                ((_, '=', _), r) => {
                    ranges.push((r, input.clone()));
                    break;
                }
                _ => panic!("Invalid input"),
            }
        }
    }
    let score = accepted
        .iter()
        .map(|i| i.values().map(|(l, u)| u - l).fold(1, |a, b| a * b))
        .sum::<i64>();
    println!("Task 2: {}", score);
}
