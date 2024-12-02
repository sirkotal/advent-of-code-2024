use std::fs;

pub fn safe_reports() -> i32 {
    let mut data: Vec<Vec<i32>> = Vec::new();
    let mut res = 0;

    let file = fs::read_to_string("./data/day2/input.txt");
    for line in file.unwrap().lines() {
        let info = line.to_string();

        let level = info.split(" ").map(|r| r.parse().unwrap()).collect::<Vec<i32>>();  
        //println!("{:?}", level);
        data.push(level);
    }

    for level_info in data {
        let mut up: Vec<i32> = level_info.clone();
        up.sort();
        let down: Vec<i32> = up.iter().copied().rev().collect();
        //println!("{:?}", level_info);
        //println!("{:?}", up);
        //println!("{:?}", down);
        if check_diff(&level_info) && (level_info == up || level_info == down) {
            res += 1;
        }
        //println!("next");
    }

    res
}

fn check_diff(level: &Vec<i32>) -> bool {
    for i in 0..(level.len() - 1) {
        if (level[i] - level[i + 1]).abs() < 1 || (level[i] - level[i + 1]).abs() > 3 {
            return false;
        }
    }

    true
}