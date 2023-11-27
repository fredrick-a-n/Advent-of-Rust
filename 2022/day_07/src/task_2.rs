use std::collections::LinkedList;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ffolder{
    name: String,
    files: LinkedList<Ffile>,
    sub_folders: LinkedList<Ffolder>,
}

impl Ffolder{
    pub fn new(name: String) -> Self {
        Self {
            name,
            files: LinkedList::new(),
            sub_folders: LinkedList::new()
        }
    }
    pub fn add_folder(&mut self, name: String) {
        if self.sub_folders.iter().filter(|x| x.name == name).peekable().peek().is_none() {
            self.sub_folders.push_back(Ffolder::new(name));
        }
    }
    pub fn add_file(&mut self, name: String ,size: i32) {
        if self.files.iter().filter(|x| x.name == name).peekable().peek().is_none() {
            self.files.push_back(Ffile::new(name, size));
        }
    }
    fn get_path(&mut self, path: &LinkedList<String>) -> &mut Self {
        let mut current = self;
        for part in path {
            current = current
                .sub_folders
                .iter_mut()
                .find(|f| f.name == part.to_string())
                .unwrap();
        }

        current
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ffile{
    name: String,
    size: i32,
}

impl Ffile{
    pub fn new(name: String, size: i32) -> Self {
        Self {
            name,
            size
        }
    }
}

fn sum_folder(f: &Ffolder) -> i32{
    return f.sub_folders.iter().map(|x| sum_folder(x)).sum::<i32>() + f.files.iter().map(|x| x.size).sum::<i32>();
}

fn sizes(f: &Ffolder, min: i32) -> Vec<i32>{
    let mut k: Vec<i32> = Vec::new();
    let n = sum_folder(f);
    if  n >= min {
        k.push(n.clone());
    } 
    for i in &f.sub_folders{
        k.append(&mut sizes(&i, min));
    }
    return k;
}

pub fn task_2() {
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut coms: Vec<&str> = contents.split("$ ").collect();
    coms.remove(0);
    let k = coms.remove(0).split(" ");

    let mut source: Ffolder = Ffolder::new(k.last().unwrap().to_string());
    

    let mut path: LinkedList<String> = LinkedList::new();
    
    
    for com in coms.iter(){
        let l = com.split_at(2);
        match l.0{
            "cd" => {
                let aasd = l.1.to_string();
                let name = aasd.trim();
                if name == ".." {
                    path.pop_back();
                } else if name == "/" { 
                    path.clear();
                } else {
                    let parent = source.get_path(&path);
                    parent.add_folder( name.to_string());
                    path.push_back(name.to_string());
                    
                }
            },
            "ls" => {
                let re = Regex::new(r"([0-9]+) ([a-z.]+)").unwrap();
                for cap in re.captures_iter(l.1){
                    source.get_path(&path).add_file(cap[2].to_string(), cap[1].parse::<i32>().unwrap());
                }
            },
            _ => panic!("invalid input"),
        }
    }

    let mut a = sizes(&source, sum_folder(&source) - 40000000);
    a.sort();
    
    println!("{}", a.first().unwrap());
    
}




