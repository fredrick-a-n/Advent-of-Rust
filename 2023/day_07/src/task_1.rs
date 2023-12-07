pub fn task_1() {
    println!("Tasks 1: {}", itertools::Itertools::sorted(include_str!("../input1").split("\n").map(|line| {
        map_to_hand(line.split_once(" ").unwrap())
    })).enumerate().fold(0, |acc, (i, (_, bet))| acc + (i as i32+1)*bet))
}   

fn map_to_hand((hand, bet) :(&str,&str)) -> (Hand, i32){
    (Hand::new(hand.chars().map(|c| match c {'T' => 10, 'J' => 11, 'Q' => 12, 'K' => 13, 'A' => 14, _ => c.to_digit(10).unwrap()}).collect()), bet.parse::<i32>().unwrap())
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
        match get_max_counts(&cards) {
            (5, _) => Hand::FiveofaKind(get_hexa(&cards)),
            (4, _) => Hand::FourofaKind(get_hexa(&cards)),
            (3, 2) => Hand::FullHouse(get_hexa(&cards)),
            (3, 1) => Hand::ThreeofaKind(get_hexa(&cards)),
            (2, 2) => Hand::TwoPair(get_hexa(&cards)),
            (2, 1) => Hand::OnePair(get_hexa(&cards)),
            _ => Hand::HighCard(get_hexa(&cards)),
        }
    }
}

fn get_max_counts(cards: &Vec<u32>) -> (u8, u8) {
    (2..15).rev().map(|i| cards.iter().filter(|&x| *x == i).count() as u8)
        .fold((0,0), |acc,j| if j > acc.0 {(j, acc.0)} else if j > acc.1 {(acc.0, j)} else {acc})
}

fn get_hexa(cards: &Vec<u32>) -> u32 {
    cards.iter().rev().enumerate().fold(0, |acc, (i, &x)| acc + x*(16_u32).pow(i as u32))
}