use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Number(usize),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => {
                            if let Some(o) = a.partial_cmp(b) {
                                if o != Ordering::Equal {
                                    return Some(o);
                                }
                            }
                        }
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (None, None) => return Some(Ordering::Equal),
                    }
                }
            }
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (a, Packet::Number(b)) => a.partial_cmp(&Packet::List(vec![Packet::Number(*b)])),
            (Packet::Number(a), b) => Packet::List(vec![Packet::Number(*a)]).partial_cmp(b),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Packet {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("10", "a");
        let mut packet = Packet::List(Vec::new());
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                ',' => (),
                '[' => {
                    let mut sub_s = String::new();
                    let mut d = 1;
                    while d > 0 {
                        let cc = chars.next().unwrap();
                        match cc {
                            '[' => d += 1,
                            ']' => d -= 1,
                            _ =>  (),
                        }
                        sub_s.push(cc);
                    }
                    if let Ok(p) = sub_s[0..sub_s.len() - 1].parse() {
                        if let Packet::List(list) = &mut packet {
                            list.push(p);
                        }
                    }
                }
                _ => {
                    if let Packet::List(list) = &mut packet {
                        list.push(Packet::Number(c.to_digit(11).unwrap() as usize));
                    }
                }
            }
        }
        Ok(packet)
    }
    type Err = ();    
}

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");

    let mut packets: Vec<Packet> = contents.split("\n\n").flat_map(|x| x.split("\n")).map(|x| x.parse::<Packet>().unwrap()).collect();
    packets.push("[[2]]".parse().unwrap());
    packets.push("[[6]]".parse().unwrap());
    packets.sort();

    let mut sum = 1;
    let mut i = 1;
    for p in packets {
        if p == "[[2]]".parse().unwrap() {
            sum *= i;
        } else if p == "[[6]]".parse().unwrap() {
            sum *= i;
        }
        i += 1;
    }
    println!("{}", sum);    

}