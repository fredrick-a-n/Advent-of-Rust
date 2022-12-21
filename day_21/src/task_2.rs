use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]

enum Monkey{
    // Name, value
    Number(String, f64),
    // Name, operand 1, operand 2, operation
    Operation(String, String, String, String),
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() < 3 {
            let mut name = parts[0].to_string();
            name.pop();
            return Ok(Monkey::Number(name, parts[1].parse().unwrap()));
        } else {
            let mut name = parts[0].to_string();
            name.pop();
            return Ok(Monkey::Operation(name, parts[1].to_string(), parts[3].to_string(), parts[2].to_string()));
        }
    }
}

impl Monkey {
    fn value_of(&self, monkeys: &HashMap<String, Monkey>) -> f64{
        match self {
            Monkey::Number(_, val) => return *val,
            Monkey::Operation(_, op1, op2, operation) => {
                match operation.chars().next().unwrap() {
                    '+' => return monkeys.get(op1).unwrap().value_of(monkeys) + monkeys.get(op2).unwrap().value_of(monkeys),
                    '-' => return monkeys.get(op1).unwrap().value_of(monkeys) - monkeys.get(op2).unwrap().value_of(monkeys),
                    '*' => return monkeys.get(op1).unwrap().value_of(monkeys) * monkeys.get(op2).unwrap().value_of(monkeys),
                    '/' => return monkeys.get(op1).unwrap().value_of(monkeys) / monkeys.get(op2).unwrap().value_of(monkeys),
                    _ => return 0.0
                };
            }
        }
    }

    fn name_of(&self) -> &str {
        match self {
            Monkey::Number(name, _) =>  name,
            Monkey::Operation(name, _, _, _) => name
        }
    }

    fn change_sign(&self, operation: &str) -> Option<Monkey>{
        if let Monkey::Operation(name, op1, op2, _) = self {
            Some(Monkey::Operation(name.to_string(), op1.to_string(), op2.to_string(), operation.to_string()))
        } else {
            None
        }
    }

}


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for l in contents.lines(){
        let mut monkey: Monkey = l.parse().unwrap();
        if monkey.name_of() == "root"{
            monkey = monkey.change_sign("-").unwrap();
        }
        if monkey.name_of() != "humn"{
            monkeys.insert(monkey.name_of().to_string(), monkey);            
        } 
    }
    let mut eq_val = 0.0;
    let mut upper = i64::max_value() as f64;
    let mut lower = 0.0;

    loop {
        if upper == lower {
            eq_val = upper;
            break;
        }
        let middle = (upper + lower) / 2.0;
        //println!("{}, {}, {}", lower, middle, upper);

        let k: Vec<f64> = [lower, middle, upper].iter().map(|x| {
            monkeys.remove("humn");
            monkeys.insert("humn".to_string(), Monkey::Number("humn".to_string(), *x));
            monkeys.get("root").unwrap().value_of(&monkeys)
        }).collect();
        //println!("{}, {}, {}", k[0], k[1], k[2]);

        if k[1] == 0.0 {
            eq_val = middle;
            break;
        } else if k[0] > k[1] || k[2] < k[1]{
            if k[1] < 0.0 {
                upper = middle;
            } else {
                lower = middle;
            }
        } else {
            if k[1] < 0.0 {
                lower = middle;
            } else {
                upper = middle;
            }
        }
    }

    println!("{}", eq_val);

}
