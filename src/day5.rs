use std::fs;
use regex::Regex;

pub fn check_safety_updates() -> i32 {
    let mut data_rules: Vec<String> = Vec::new();
    let mut data_updates: Vec<String> = Vec::new();
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut safe_updates: Vec<String> = Vec::new();
    let mut res: i32 = 0;

    let reg = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();

    let file = fs::read_to_string("./data/day5/test.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        if info.contains("|") {
            data_rules.push(info);
        }
        else {
            data_updates.push(info)
        }
    }

    // println!("{:?}", data_rules.len());

    let updated_rules = data_rules.join(" ");

    // println!("{updated_rules}");

    for cap in reg.captures_iter(&updated_rules) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        rules.push((a, b));
        // println!("Match: {} and {}", &cap[1], &cap[2]);
    }

    middle_page_number_sum(safe_updates, &mut res);
    res
}

fn middle_page_number_sum(updates: Vec<String>, sum: &mut i32) {
    for update in &updates {
        let filtered_update = update.split(",").map(|p| p.parse().unwrap()).collect::<Vec<i32>>();
        let vec_index = (updates.len() / 2) + (updates.len() % 2);
        *sum += filtered_update[vec_index];
    }
}