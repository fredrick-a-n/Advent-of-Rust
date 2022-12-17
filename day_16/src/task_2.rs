use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::path::Path;
use regex::Regex;


pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut valves: HashMap<String, i32> = HashMap::new();
    let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
    let re= r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)".to_owned();    
    for cap in Regex::new(&re).unwrap().captures_iter(&contents){
        let flow: i32 = cap[2].parse().unwrap();
        let name = cap[1].to_string();
        valves.insert(name.to_string(), flow);
        tunnels.insert(name.to_string(), cap[3].split(", ").map(|x| x.to_string()).collect());
    }

    let mut dists: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut indices: HashMap<String, i32> = HashMap::new();

    for v in valves.clone() {
        if v.0 != "AA" && v.1 == 0 {
            continue;
        }

        if v.0 != "AA" {
            indices.insert(v.0.to_string(), indices.len() as i32);
        }

        dists.insert(v.0.to_string(), HashMap::new());
        dists.get_mut(&v.0.to_string()).unwrap().insert(v.0.to_string(), 0);
        dists.get_mut(&v.0.to_string()).unwrap().insert("AA".to_string(), 0);
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(v.0.to_string());
        
        let mut queue: LinkedList<(String, i32)> = LinkedList::new();
        queue.push_back((v.0.to_string(), 0));

        while queue.len() > 0 {
            let (pos, dis) = queue.pop_front().unwrap();
            for neighbour in tunnels.get(&pos).unwrap() {
                if visited.contains(neighbour) {
                    continue;
                }
                visited.insert(neighbour.to_string());
                if valves[neighbour] != 0{
                    dists.get_mut(&v.0.to_string()).unwrap().insert(neighbour.to_string(), dis + 1);
                }
                queue.push_back((neighbour.to_string(), dis + 1));
            }
        }
        dists.get_mut(&v.0.to_string()).unwrap().remove(&v.0.to_string());
        if v.0 != "AA" {
            dists.get_mut(&v.0.to_string()).unwrap().remove(&"AA".to_string());
        }
    }

    let mut cache: HashMap<(String, i32, i32), i32> = HashMap::new();

    let mut max = 0;

    for i in 0..((1 << indices.len()) / 2) {
        max = max.max(dfs(&mut dists, "AA".to_string(), 26, i, &mut cache, &mut indices, &mut valves) + dfs(&mut dists, "AA".to_string(), 26, i ^ ((1 << indices.len()) - 1), &mut cache, &mut indices, &mut valves));
    }

    println!("{}", max)

}


fn dfs(dists:&mut HashMap<String, HashMap<String, i32>>, valve: String, time: i32, mask: i32, 
cache:&mut HashMap<(String, i32, i32), i32>, indices:&mut HashMap<String, i32>, valves: &mut HashMap<String, i32>) -> i32 {
    if cache.contains_key(&(valve.clone(), time, mask)){
        return cache.get(&(valve.clone(), time, mask)).unwrap().clone();
    }

    let mut max = 0;
    for neighbour in dists.clone().get(&valve).unwrap() {
        let bit = 1 << indices.get(neighbour.0).unwrap();
        if mask & bit != 0 {
            continue;
        }
        let remaining = time - dists.get(&valve).unwrap().get(neighbour.0).unwrap() - 1;
        if remaining <= 0 {
            continue;
        }
        max = max.max(dfs(dists, neighbour.0.to_string(), remaining, mask | bit, cache, indices, valves) + valves.get(neighbour.0).unwrap() * remaining)
    }
    cache.insert((valve, time, mask), max);
    return max;
}