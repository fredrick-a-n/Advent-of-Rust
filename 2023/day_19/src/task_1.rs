use std::collections::HashMap;

pub fn task_1() {
    let (transforms, input) = include_str!("../input1").split_once("\n\n").unwrap();
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
    let input = input
        .lines()
        .map(|l| {
            l[1..(l.len() - 1)]
                .split(",")
                .map(|c| c.split_once("=").unwrap())
                .map(|(k, v)| {
                    (
                        k.chars().next().unwrap_or('?'),
                        v.parse::<i64>().unwrap_or(0),
                    )
                })
                .collect::<HashMap<char, i64>>()
        })
        .collect::<Vec<HashMap<char, i64>>>();

    let mut accepted: Vec<HashMap<char, i64>> = Vec::new();
    for inp in input {
        let mut key = "in";
        while key != "A" && key != "R" {
            let transform = transforms.get(key).unwrap();
            for t in transform {
                match *t {
                    ((c, '>', v), r) => {
                        if v < *inp.get(&c).unwrap() {
                            key = r;
                            break;
                        }
                    }
                    ((c, '<', v), r) => {
                        if v > *inp.get(&c).unwrap() {
                            key = r;
                            break;
                        }
                    }
                    ((_, '=', _), r) => {
                        key = r;
                        break;
                    }
                    _ => panic!("Invalid input"),
                }
            }
        }
        if key == "A" {
            accepted.push(inp.clone());
        }
    }
    let score = accepted
        .iter()
        .map(|i| i.values().sum::<i64>())
        .sum::<i64>();
    println!("Task 1: {}", score);
}
