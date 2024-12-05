use std::fs;
use regex::Regex;

pub fn check_safety_updates() -> i32 {
    let mut data_rules: Vec<String> = Vec::new();
    let mut data_updates: Vec<String> = Vec::new();
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut safe_updates: Vec<String> = Vec::new();
    let mut res: i32 = 0;

    let reg = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();

    let file = fs::read_to_string("./data/day5/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        if info.contains("|") {
            data_rules.push(info);
        }
        else {
            if info != "" {
                data_updates.push(info);
            }
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

    //println!("{:?}", data_updates);

    for update in data_updates {
        let mut flag: bool = true;
        for pair in &rules {
            if update.contains(&pair.0.to_string()) && update.contains(&pair.1.to_string()) {
                let index_zero = update.find(&pair.0.to_string()).unwrap();
                let index_one = update.find(&pair.1.to_string()).unwrap();

                if index_zero >= index_one {
                    flag = false;
                }
            }
        }
        if flag {
            safe_updates.push(update.clone());
        }

    }

    //println!("{:?}", safe_updates);

    middle_page_number_sum(safe_updates, &mut res);
    res
}

fn middle_page_number_sum(updates: Vec<String>, sum: &mut i32) {
    for update in &updates {
        let filtered_update = update.split(",").map(|p| p.parse().unwrap()).collect::<Vec<i32>>();
        let vec_index = (filtered_update.len() / 2) + (filtered_update.len() % 2) - 1;
        //println!("{vec_index}");
        *sum += filtered_update[vec_index];
    }
}

pub fn fix_unsafe_updates() -> i32 {
    let mut data_rules: Vec<String> = Vec::new();
    let mut data_updates: Vec<String> = Vec::new();
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut unsafe_updates: Vec<String> = Vec::new();
    let mut res: i32 = 0;

    let reg = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();

    let file = fs::read_to_string("./data/day5/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        if info.contains("|") {
            data_rules.push(info);
        }
        else {
            if info != "" {
                data_updates.push(info);
            }
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

    //println!("{:?}", data_updates);

    for update in data_updates {
        let mut flag: bool = true;
        let mut count = 0;
        let mut new_update = update.clone();

        while flag {
            flag = false;
            for pair in &rules {
                if new_update.contains(&pair.0.to_string()) && new_update.contains(&pair.1.to_string()) {
                    let index_zero = new_update.find(&pair.0.to_string()).unwrap();
                    let index_one = new_update.find(&pair.1.to_string()).unwrap();
    
                    if index_zero >= index_one {
                        flag = true;
                        count += 1;
                        new_update = apply_rules(new_update, pair.0, pair.1);
                    }
                }
            }
        }

        if count != 0 {
            unsafe_updates.push(new_update);
        }
    }

    //println!("final is {:?}", unsafe_updates);

    middle_page_number_sum(unsafe_updates, &mut res);
    res
}

fn apply_rules(update: String, i: i32, j: i32) -> String {
    let mut filtered_update = update.split(",").map(|p| p.parse().unwrap()).collect::<Vec<i32>>();
    let index_zero = filtered_update.iter().position(|&n| n == i).unwrap();
    let index_one = filtered_update.iter().position(|&n| n == j).unwrap();
    filtered_update.swap(index_zero, index_one);

    let final_update = filtered_update.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");

    //println!("{:?}", final_update);

    final_update
}