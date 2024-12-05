use std::fs;
use regex::Regex;

pub fn search_xmas() -> i32 {
    let mut data: Vec<String> = Vec::new();
    let mut res: i32 = 0;

    let reg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let file = fs::read_to_string("./data/day3/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        data.push(info);
    }

    println!("{:?}", data.len());

    for cap in reg.captures_iter(&joined_data) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        res += a * b;
        println!("Match: {} and {}", &cap[1], &cap[2]);
    }

    res
}