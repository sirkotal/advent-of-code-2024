use std::fs;
use regex::Regex;

pub fn fix_memory() -> i32 {
    let mut data: Vec<String> = Vec::new();
    let mut res: i32 = 0;

    let reg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let file = fs::read_to_string("./data/day03/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        data.push(info);
    }

    println!("{:?}", data.len());

    let mut joined_data = data.join("");

    // println!("{:?}", data);

    for cap in reg.captures_iter(&joined_data) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        res += a * b;
        println!("Match: {} and {}", &cap[1], &cap[2]);
    }

    res
}

pub fn fix_memory_complete() -> i32 {
    let mut data: Vec<String> = Vec::new();
    let mut res: i32 = 0;
    let mut flag: bool = true;

    let reg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let burner = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();

    let file = fs::read_to_string("./data/day03/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        data.push(info);
    }

    //println!("{:?}", data.len());

    let mut joined_data = data.join("");

    //println!("{:?}", joined_data);

    let final_data = burner.replace_all(&joined_data, "");

    //println!("{:?}", final_data);

    for cap in reg.captures_iter(&final_data) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        res += a * b;
        //println!("Match: {} and {}", &cap[1], &cap[2]);
    }

    res
}