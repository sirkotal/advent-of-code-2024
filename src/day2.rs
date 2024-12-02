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

pub fn safe_reports_dampened() -> i32 {
    let mut data: Vec<Vec<i32>> = Vec::new();
    let mut res = 0;

    let file = fs::read_to_string("./data/day2/input.txt");
    for line in file.unwrap().lines() {
        let info = line.to_string();

        let level = info.split(" ").map(|r| r.parse().unwrap()).collect::<Vec<i32>>();  
        data.push(level);
    }

    for level_info in data {
        if check_diff_dampened(&level_info) < 1 {
            //println!("{:?}", level_info);
            res += 1;
        }
        else {
            //println!("here");
            if safety_check(&level_info) {
                res += 1;
            }
        }
    }

    res
}

fn check_diff_dampened(level: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut up: Vec<i32> = level.clone();
    up.sort();
    let down: Vec<i32> = up.iter().copied().rev().collect();
    //println!("normal {:?}", level);
    //println!("up {:?}", up);
    //println!("down {:?}", down);
    if *level != up && *level != down {
        count += 1;
    }
    for i in 0..(level.len() - 1) {
        if (level[i] - level[i + 1]).abs() < 1 || (level[i] - level[i + 1]).abs() > 3 {
            count += 1;
        }
    }
    //println!("count here: {count}");
    count
}

fn safety_check(level: &Vec<i32>) -> bool {
    let mut sublevels: Vec<Vec<i32>> = Vec::new();
    let mut checks: Vec<i32> = Vec::new();

    for i in 0..level.len() {
        let mut carbon: Vec<i32> = level.clone();
        carbon.remove(i);
        let sublevel: Vec<i32> = carbon.clone();
        //println!("{:?}", sublevel);
        sublevels.push(sublevel);
    }

    for i in 0..sublevels.len() {
        //println!("{:?}", sublevels[i]);
        checks.push(check_diff_dampened(&sublevels[i]));
    }

    //println!("{:?}", checks);

    if checks.contains(&0) {
        return true;
    }

    false
}