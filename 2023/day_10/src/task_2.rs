pub fn task_2() {
    let mut k = include_str!("../input1")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let s = k
        .iter()
        .enumerate()
        .find(|(_, x)| x.contains(&'S'))
        .unwrap();
    let s = (s.0, s.1.iter().position(|&x| x == 'S').unwrap());
    let mut pos = s;
    let mut pipes: Vec<Vec<char>> = vec![vec!['o'; k[0].len()]; k.len()];
    k[pos.0][pos.1] = translate_start(pos, &k);
    let mut last_pos = original_last(pos, &k);
    loop {
        pipes[pos.0][pos.1] = k[pos.0][pos.1];
        let n = find_next(pos, last_pos, &k);
        if n == s {
            break;
        }
        last_pos = pos;
        pos = n;
    }
    for line in pipes.iter() {
        println!(
            "{}",
            line.iter()
                .map(|x| if *x != 'o' { '|' } else { 'o' })
                .collect::<String>()
        );
    }
    let mut sum = 0;
    for i in 0..k.len() {
        for j in 0..k[i].len() {
            if pipes[i][j] == 'o' {
                sum += (0..j)
                    .map(|o| {
                        if pipes[i][o] == '|' || pipes[i][o] == 'J' || pipes[i][o] == 'L' {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
                    % 2;
            }
        }
    }
    println!("Task 2: {}", sum)
}

fn find_next(pos: (usize, usize), last: (usize, usize), k: &Vec<Vec<char>>) -> (usize, usize) {
    match (
        k[pos.0][pos.1],
        (pos.0 as i32 - last.0 as i32, pos.1 as i32 - last.1 as i32),
    ) {
        ('J', (0, 1)) => return (pos.0 - 1, pos.1),
        ('J', (1, 0)) => return (pos.0, pos.1 - 1),
        ('L', (0, -1)) => return (pos.0 - 1, pos.1),
        ('L', (1, 0)) => return (pos.0, pos.1 + 1),
        ('F', (0, -1)) => return (pos.0 + 1, pos.1),
        ('F', (-1, 0)) => return (pos.0, pos.1 + 1),
        ('7', (0, 1)) => return (pos.0 + 1, pos.1),
        ('7', (-1, 0)) => return (pos.0, pos.1 - 1),
        ('|', (1, 0)) => return (pos.0 + 1, pos.1),
        ('|', (-1, 0)) => return (pos.0 - 1, pos.1),
        ('-', (0, 1)) => return (pos.0, pos.1 + 1),
        ('-', (0, -1)) => return (pos.0, pos.1 - 1),
        _ => {}
    }
    unreachable!("No next found")
}

pub fn translate_start(pos: (usize, usize), k: &Vec<Vec<char>>) -> char {
    let mut v = (0, 0, 0, 0);
    match k[pos.0 + 1][pos.1] {
        'J' | '|' | 'L' => v.0 = 1,
        _ => {}
    }
    if pos.0 > 0 {
        match k[pos.0 - 1][pos.1] {
            'F' | '|' | '7' => v.1 = 1,
            _ => {}
        }
    }
    match k[pos.0][pos.1 + 1] {
        'J' | '-' | '7' => v.2 = 1,
        _ => {}
    }
    if pos.1 > 0 {
        match k[pos.0][pos.1 - 1] {
            'L' | '-' | 'F' => v.3 = 1,
            _ => {}
        }
    }
    match v {
        (1, 1, 0, 0) => return '|',
        (0, 1, 0, 1) => return 'J',
        (0, 1, 1, 0) => return 'L',
        (1, 0, 1, 0) => return 'F',
        (1, 0, 0, 1) => return '7',
        (0, 0, 1, 1) => return '-',
        _ => {}
    }

    unreachable!("No next found")
}

fn original_last(pos: (usize, usize), k: &Vec<Vec<char>>) -> (usize, usize) {
    match k[pos.0][pos.1] {
        'J' => return (pos.0 - 1, pos.1),
        'L' => return (pos.0 - 1, pos.1),
        'F' => return (pos.0 + 1, pos.1),
        '7' => return (pos.0 + 1, pos.1),
        '|' => return (pos.0 + 1, pos.1),
        '-' => return (pos.0, pos.1 - 1),
        _ => {}
    }
    unreachable!("No last found")
}
