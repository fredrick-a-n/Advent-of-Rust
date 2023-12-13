pub fn task_1() {
    let groups = include_str!("../input1")
        .split("\n\n")
        .map(|group| {
            (
                group
                    .lines()
                    .map(|line| {
                        line.chars().enumerate().fold(0, |acc, (i, c)| {
                            acc + (c == '#') as u32 * 2u32.pow(i as u32)
                        })
                    })
                    .collect::<Vec<u32>>(),
                (0..(group.lines().next().unwrap().len()))
                    .map(|i| {
                        group
                            .lines()
                            .map(|line| line.chars().nth(i).unwrap())
                            .enumerate()
                            .fold(0, |acc, (j, c)| {
                                acc + (c == '#') as u32 * 2u32.pow(j as u32)
                            })
                    })
                    .collect::<Vec<u32>>(),
            )
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>();

    let k = groups
        .iter()
        .map(|(r, c)| (check_mask(r.clone()), check_mask(c.clone())))
        .fold(0, |acc, (r, c)| acc + 100 * r + c);
    println!("Task 1: {}", k);
}

fn check_mask(mut v: Vec<u32>) -> usize {
    let m = apply_mask(v.clone());
    if m > 0 {
        return m;
    }
    v.reverse();
    let m = apply_mask(v.clone());
    if m > 0 {
        return v.len() - m;
    }
    0
}

fn apply_mask(v: Vec<u32>) -> usize {
    let mut mask = 0;
    for (i, j) in v.clone().iter().enumerate() {
        mask ^= j;
        if mask == 0 && i > 0 {
            let s = (i + 1) / 2;
            if comp_vec(
                v[0..s].iter().rev().collect::<Vec<&u32>>(),
                v[s..(i + 1)].to_vec(),
            ) {
                return s;
            }
        }
    }
    return 0;
}

fn comp_vec(v1: Vec<&u32>, v2: Vec<u32>) -> bool {
    v1.iter().zip(v2.iter()).all(|(a, b)| *a == b)
}
