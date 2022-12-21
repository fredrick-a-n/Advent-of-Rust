use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

enum Monkey{
    // Name, value
    Number(String, i128),
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
    fn value_of(&self, monkeys: &HashMap<String, Monkey>) -> i128{
        match self {
            Monkey::Number(_, val) => return *val,
            Monkey::Operation(_, op1, op2, operation) => {
                match operation.chars().next().unwrap() {
                    '+' => return monkeys.get(op1).unwrap().value_of(monkeys) + monkeys.get(op2).unwrap().value_of(monkeys),
                    '-' => return monkeys.get(op1).unwrap().value_of(monkeys) - monkeys.get(op2).unwrap().value_of(monkeys),
                    '*' => return monkeys.get(op1).unwrap().value_of(monkeys) * monkeys.get(op2).unwrap().value_of(monkeys),
                    '/' => return monkeys.get(op1).unwrap().value_of(monkeys) / monkeys.get(op2).unwrap().value_of(monkeys),
                    _ => return 0
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

}


pub fn task_1() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for l in contents.lines(){
        let monkey: Monkey = l.parse().unwrap();
        monkeys.insert(monkey.name_of().to_string(), monkey);
    }

    println!("{}", monkeys.get("root").unwrap().value_of(&monkeys));

}
