use std::collections::LinkedList;
use std::fs;
use std::path::Path;
use regex::Regex;



pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let data: Vec<&str> = contents.split("\n\n").collect();


    //init data
    let mut re= r"(\[[A-Z]\]|\s{3})".to_owned();    
    let s = data.get(0).unwrap();
    let mut asd: Vec<&str> = s.split("\n").collect();
    let line_num = asd.pop().unwrap().trim().chars().next_back().unwrap().to_digit(10).unwrap() ;
    let mut boxes: Vec<LinkedList<char>> = vec![LinkedList::new(); line_num as usize];
    for _i in 1..line_num{
        re.push_str(r"\s(\[[A-Z]\]|\s{3})");
        
    }
    re.push_str(r"\n");
    for cap in Regex::new(&re).unwrap().captures_iter(&s){
        for i in 0..line_num{
            if cap[(i+1) as usize].to_string().trim() != "" {
                let b: Vec<char> = cap[(i+1) as usize].chars().collect();
            
                boxes.get_mut(i as usize).unwrap().push_front(b.get(1).unwrap().clone());
            }
        }
    }


    //the algo
    let re2 = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    for cap in re2.captures_iter(data.get(1).unwrap()){
        let n = cap[1].parse::<usize>().unwrap();
        let kek = boxes.get_mut(cap[2].parse::<usize>().unwrap()-1).unwrap();
            let t = &mut kek.split_off(kek.len()-n);
        for i in t{
            boxes.get_mut(cap[3].parse::<usize>().unwrap()-1).unwrap().push_back(i.clone());
        }
    }

    for i  in 0..line_num{
        let k = boxes.get(i as usize).unwrap();
        print!("{}", k.back().unwrap_or(&' '));
    }
    print!("\n");

}