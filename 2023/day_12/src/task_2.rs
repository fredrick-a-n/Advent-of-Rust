use std::collections::HashMap;
pub fn task_2() {
    println!(
        "Task 2: {}",
        include_str!("../input1")
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(springs, nums)| {
                (
                    springs.chars().collect::<Vec<char>>(),
                    nums.split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                )
            })
            .map(|(springs, nums)| (vec![springs; 5].join(&'?'), vec![nums; 5].concat()))
            .map(|(springs, nums)| rec(&mut HashMap::new(), (&springs, &nums), 0, 0, 0))
            .sum::<i128>()
    )
}
fn rec(
    mm: &mut HashMap<(usize, usize, usize), i128>,
    (springs, nums): (&Vec<char>, &Vec<usize>),
    i: usize,
    j: usize,
    k: usize,
) -> i128 {
    if mm.contains_key(&(i, j, k)) {
        mm.get(&(i, j, k)).unwrap().clone()
    } else if i == springs.len() {
        if (j == nums.len() && k == 0) || (j == nums.len() - 1 && nums[j] == k) {
            1
        } else {
            0
        }
    } else {
        let sum = vec!['.', '#']
            .iter()
            .filter(|&&c| springs[i] == c || springs[i] == '?')
            .map(|&c| match c {
                '.' => {
                    if k == 0 {
                        rec(mm, (springs, nums), i + 1, j, 0)
                    } else if k > 0 && j < nums.len() && nums[j] == k {
                        rec(mm, (springs, nums), i + 1, j + 1, 0)
                    } else {
                        0
                    }
                }
                '#' => rec(mm, (springs, nums), i + 1, j, k + 1),
                _ => unreachable!(),
            })
            .sum::<i128>();
        mm.insert((i, j, k), sum);
        sum
    }
}
