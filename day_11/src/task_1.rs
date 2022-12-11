use std::collections::LinkedList;
use std::fs;
use std::path::Path;
use regex::Regex;

struct Monkey {
    items: LinkedList<i128>,
    operation: String,
    test: i128,
    true_op: i128,
    false_op: i128,
}

impl Monkey{
    pub fn new(items: LinkedList<i128>,
        operation: String,
        test: i128,
        true_op: i128,
        false_op: i128) -> Self {
        Self {
            items: items.to_owned(),
            operation: operation.to_string(),
            test,
            true_op,
            false_op
        }
    }

}

pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let re= r"Monkey \d+:\n  Starting items: ([\d\s,]+)\n  Operation: new = old ([\d+*-/\w\s]+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)".to_owned();    
    for cap in Regex::new(&re).unwrap().captures_iter(&contents) {
        let items = cap[1].split(", ").map(|x| x.parse::<i128>().unwrap()).collect(); 
        monkeys.push(Monkey::new(items, cap[2].to_string(), cap[3].parse::<i128>().unwrap(), cap[4].parse::<i128>().unwrap(), cap[5].parse::<i128>().unwrap()));
    }
    let mut item_counter: Vec<i32> = vec![0; monkeys.len()];
    for _n in 0..20 {
        for j in 0..monkeys.len() {
            for _i in 0..monkeys.get(j).unwrap().items.len() {
                let mut item = monkeys.get_mut(j).unwrap().items.pop_front().unwrap();
                let o: Vec<&str> = monkeys.get(j).unwrap().operation.split(" ").collect();
                let ad: i128;
                match *o.get(1).unwrap(){
                    "old" => ad = item,
                    _ => ad = o.get(1).unwrap().parse::<i128>().unwrap(),
                }
                match *o.get(0).unwrap(){
                    "*" => item = (item * ad)/3,
                    "+" => item = (item + ad)/3,
                    _ => panic!("Invalid input")
                }
                if item%monkeys.get(j).unwrap().test == 0{
                    let dest = monkeys.get_mut(j).unwrap().true_op as usize;
                    monkeys.get_mut(dest).unwrap().items.push_back(item.clone());
                } else {
                    let dest = monkeys.get_mut(j).unwrap().false_op as usize;
                    monkeys.get_mut(dest).unwrap().items.push_back(item.clone());
                }
                item_counter[j] += 1;
            }
        }
    }
    item_counter.sort();
    println!("{}", item_counter.pop().unwrap() * item_counter.pop().unwrap())
}