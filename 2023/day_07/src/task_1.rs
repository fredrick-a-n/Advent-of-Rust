use itertools::Itertools;
pub fn task_1() {
    println!("Tasks 1: {}", include_str!("../input1").split("\n").map(|line| {
        let (h, s) = line.split_once(" ").unwrap();
        (Hand::new(h.chars().map(|c| match c {'T' => 10, 'J' => 11, 'Q' => 12, 'K' => 13, 'A' => 14, _ => c.to_digit(10).unwrap()}).collect()), 
        s.parse::<i32>().unwrap())
    }).sorted().enumerate().fold(0, |acc, (i, (_, bet))| acc + (i as i32+1)*bet))
}   

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum Hand {
    HighCard(u32),
    OnePair(u32),
    TwoPair(u32),
    ThreeofaKind(u32),
    FullHouse(u32),
    FourofaKind(u32),
    FiveofaKind(u32),
}

impl Hand {
    fn new(cards: Vec<u32>) -> Hand {
        let num = cards.iter().rev().enumerate().fold(0, |acc, (i, &x)| acc + x*(16_u32).pow(i as u32));
        match (2..15).rev().map(|i| cards.iter().filter(|&x| *x == i).count() as u8)
            .fold((0,0), |acc,j| if j > acc.0 {(j, acc.0)} else if j > acc.1 {(acc.0, j)} else {acc}) {
            (5, _) => Hand::FiveofaKind(num),
            (4, 1) => Hand::FourofaKind(num),
            (3, 2) => Hand::FullHouse(num),
            (3, 1) => Hand::ThreeofaKind(num),
            (2, 2) => Hand::TwoPair(num),
            (2, 1) => Hand::OnePair(num),
            _ => Hand::HighCard(num),
        }
    }
}