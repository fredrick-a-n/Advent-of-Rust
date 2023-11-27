use std::fs;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

const MAX_TIME: i32 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BluePrint {
    costs: [[i32; 3]; 4],
}

impl FromStr for BluePrint {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //                Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
        let re= r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.".to_owned();    
        for cap in Regex::new(&re).unwrap().captures_iter(s){
            return Ok(BluePrint {
                costs: [
                    [cap[1].parse().unwrap(), 0, 0], 
                    [cap[2].parse().unwrap(), 0, 0], 
                    [cap[3].parse().unwrap(), cap[4].parse().unwrap(), 0], 
                    [cap[5].parse().unwrap(), 0, cap[6].parse().unwrap()]
                    ]
            })
        }
        return Err(());
    }
}

fn next_buy(bp: BluePrint, resources: [i32; 3], machines: [i32; 4], time: i32, to_buy: usize) -> i32 {
    if time > MAX_TIME - 5 && machines[3] == 0{
        return 0;
    }
    if to_buy != 3 && bp.costs.iter().all(|x| x[to_buy] <= machines[to_buy] ) {
        return 0;
    }
    if (to_buy == 3 || to_buy == 2) && time < 8 {
        return 0;
    }
    let time_needed = (0..3).map(|i| 
        (bp.costs[to_buy][i] - resources[i]).checked_div(machines[i]).unwrap_or_else(|| 
        if bp.costs[to_buy][i] - resources[i] <= 0 {
            0
        } else {
            i32::max_value()
        })
    ).max().unwrap();
    if time_needed > MAX_TIME - time {
        return (MAX_TIME - time) * machines[3];
    }
    let mut time_spent:i32 = 0;
    let mut n_resources = resources.clone();
    while (time + time_spent) < MAX_TIME && time_spent < time_needed {
        n_resources[0] += machines[0];
        n_resources[1] += machines[1];
        n_resources[2] += machines[2];
        time_spent += 1;
    }
    let mut new_geodes = machines[3] * time_spent;

    if time + time_spent >= MAX_TIME{
        return new_geodes;
    }

    let mut n_machines = machines.clone();
    n_machines[to_buy] += 1;
    n_resources[0] -= bp.costs[to_buy][0];
    n_resources[1] -= bp.costs[to_buy][1];
    n_resources[2] -= bp.costs[to_buy][2];
    time_spent += 1;
    n_resources[0] += machines[0];
    n_resources[1] += machines[1];
    n_resources[2] += machines[2];
    new_geodes += machines[3];

    return [
        next_buy(bp, n_resources.clone(), n_machines.clone(), time + time_spent, 0),
        next_buy(bp, n_resources.clone(), n_machines.clone(), time + time_spent, 1),
        next_buy(bp, n_resources.clone(), n_machines.clone(), time + time_spent, 2),
        next_buy(bp, n_resources.clone(), n_machines.clone(), time + time_spent, 3)
    ].iter().max().unwrap().clone() + new_geodes;
}

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./test1"))
        .expect("Should have been able to read the file");
    let prints: Vec<BluePrint> = contents.lines().map(|x| x.parse().unwrap()).collect();
    let mut maxes: Vec<i32> = Vec::new();
    for p in prints {
        let max = [
            next_buy(p, [0; 3], [1,0,0,0], 0, 0),
            next_buy(p, [0; 3], [1,0,0,0], 0, 1)
        ].iter().max().unwrap().clone();
        maxes.push(max);
    }
    
    let mut max = 0;
    for i in 0..maxes.len() {
        max *= maxes.get(i).unwrap()
    }
    println!("{}", max);
    
}
