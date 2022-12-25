use std::{fs, ops};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Snafu {
    val: String,
    decimal_val: i128
}

impl ops::Add for Snafu {
    type Output = Snafu;
    fn add(self, rhs: Self) -> Self::Output {
        let mut carry = 0;
        let mut result = "".to_string();
        let mut s1 = self.val.chars().rev().peekable();
        let mut s2 = rhs.val.chars().rev().peekable();

        while carry != 0 || s1.peek() != None || s2.peek() != None {
            let mut val: i32 = carry;
            carry = 0;
            if s1.peek() != None {
                match s1.next().unwrap() {
                    '=' => val -= 2,
                    '-' => val -= 1,
                    c => val += c.to_digit(10).unwrap() as i32
                }
            }
            if s2.peek() != None {
                match s2.next().unwrap() {
                    '=' => val -= 2,
                    '-' => val -= 1,
                    c => val += c.to_digit(10).unwrap() as i32
                }
            }
            let c = match val {
                -5 => {carry = -1; '0' },
                -4 => {carry = -1; '1' },
                -3 => {carry = -1; '2' },
                -2 => {carry = 0; '=' },
                -1 => {carry = 0; '-' },
                0 => {carry = 0; '0' },
                1 => {carry = 0; '1' },
                2 => {carry = 0; '2' },
                3 => {carry = 1; '=' },
                4 => {carry = 1; '-' },
                5 => {carry = 1; '0' },
                _ => unreachable!()
            };
            result.push(c);
        }
        return Snafu {
            val: result.chars().rev().collect(),
            decimal_val: self.decimal_val + rhs.decimal_val
        };
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i128 = 0;
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '=' => num -= 2*(5 as i128).pow(i as u32),
                '-' => num -= 1*(5 as i128).pow(i as u32),
                '0' => (),
                '1' => num += 1*(5 as i128).pow(i as u32),
                '2' => num += 2*(5 as i128).pow(i as u32),
                _ => unreachable!()
            }
        }
        Ok(Snafu{
            val: s.to_string(),
            decimal_val: num
        })
    }
}


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let nums: Vec<Snafu> = contents.lines().map(|s| s.parse().unwrap()).collect();
    let mut sum: Snafu = "0".parse().unwrap(); 
    for num in nums {
        sum = sum + num;
    }

    println!("{}, {}", sum.decimal_val, sum.val);
}
