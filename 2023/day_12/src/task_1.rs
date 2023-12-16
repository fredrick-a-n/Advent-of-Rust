pub fn task_1() {
    let rows = include_str!("../input1")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, nums)| {
            (
                springs.chars().collect::<Vec<char>>(),
                nums.split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect::<Vec<(Vec<char>, Vec<i32>)>>();
    let mut sum = 0;
    for row in rows {
        sum += rec((&row.0, &row.1), 0);
    }
    println!("Task 1: {}", sum)
}

fn rec((springs, nums): (&Vec<char>, &Vec<i32>), i: usize) -> i32 {
    if i == springs.len() {
        return if check((springs, nums)) { 1 } else { 0 };
    }
    if springs[i] == '?' {
        let mut springs_1 = springs.clone();
        springs_1[i] = '.';
        let mut springs_2 = springs.clone();
        springs_2[i] = '#';

        return rec((&springs_1, nums), i + 1) + rec((&springs_2, nums), i + 1);
    }
    rec((springs, nums), i + 1)
}

fn check((springs, nums): (&Vec<char>, &Vec<i32>)) -> bool {
    let k = (vec![springs.clone(), vec!['o']].concat())
        .iter()
        .fold((Vec::new(), 0), |(acc, i), c| match c {
            '.' => {
                if i == 0 {
                    (acc, 0)
                } else {
                    (vec![acc, vec![i]].concat(), 0)
                }
            }
            '#' => (acc, i + 1),
            'o' => {
                if i == 0 {
                    (acc, 0)
                } else {
                    (vec![acc, vec![i]].concat(), 0)
                }
            }
            _ => unreachable!(),
        })
        .0;
    k.len() == nums.len() && k.iter().zip(nums.iter()).all(|(v, n)| *v == *n)
}
