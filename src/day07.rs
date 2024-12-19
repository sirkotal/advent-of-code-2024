use std::fs;
use itertools::Itertools;

pub fn bridge_repair() -> i64 {
    let mut data: Vec<String> = Vec::new();
    let mut eq_forms: Vec<(i64, String)> = Vec::new();
    let symbols: Vec<String> = vec!["+".to_string(), "*".to_string()];

    let mut res: i64 = 0;

    let file = fs::read_to_string("./data/day07/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        data.push(info);
    }

    for line in data {
        let processed_line = line.split(": ").map(|x| x.to_string()).collect::<Vec<String>>();
        eq_forms.push((processed_line[0].parse().unwrap(), processed_line[1].clone()));
    }


    for equation in &eq_forms {
        let eq_members = equation.1.split(" ").map(|n| n.parse().unwrap()).collect::<Vec<i64>>();
        //println!("{:?}", eq_members);
        let symbol_combinations = (0..eq_members.len() - 1).map(|_| symbols.clone()).multi_cartesian_product();
        for combo in symbol_combinations {
            //println!("{:?}", combo);
            let mut total = eq_members[0];
            for (i, symbol) in combo.iter().enumerate() {
                let next_number = eq_members[i + 1];
                if *symbol == "+" {
                    total += next_number;
                } 
                else if *symbol == "*" {
                    total *= next_number;
                }
            }
            //println!("{:?}", total);
            if total == equation.0 {
                res += equation.0;
                break;
            }
        }
    }

    //println!("{:?}", eq_forms);

    res
}

pub fn calibrated_bridge_repair() -> i64 {
    let mut data: Vec<String> = Vec::new();
    let mut eq_forms: Vec<(i64, String)> = Vec::new();
    let symbols: Vec<String> = vec!["+".to_string(), "*".to_string(), "||".to_string()];

    let mut res: i64 = 0;

    let file = fs::read_to_string("./data/day07/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        data.push(info);
    }

    for line in data {
        let processed_line = line.split(": ").map(|x| x.to_string()).collect::<Vec<String>>();
        eq_forms.push((processed_line[0].parse().unwrap(), processed_line[1].clone()));
    }


    for equation in &eq_forms {
        let mut eq_members = equation.1.split(" ").map(|n| n.parse().unwrap()).collect::<Vec<i64>>();
        //println!("{:?}", eq_members);
        let symbol_combinations = (0..eq_members.len() - 1).map(|_| symbols.clone()).multi_cartesian_product();
        for combo in symbol_combinations {
            //println!("{:?}", combo);
            let mut total = eq_members[0];
            for (i, symbol) in combo.iter().enumerate() {
                let next_number = eq_members[i + 1];
                if *symbol == "+" {
                    total += next_number;
                } 
                else if *symbol == "*" {
                    total *= next_number;
                }
                else if *symbol == "||" {
                    // total = (total.to_string() + &eq_members[i + 1].to_string()).parse().unwrap(); //total * 10 + eq_members[i + 1];
                    total = total * 10i64.pow(eq_members[i + 1].ilog10() + 1) + eq_members[i + 1]
                }

                /*if eq_members.len() == 4 {
                    println!("marcus give me the final count {:?}", total);
                }*/
            }
            if total == equation.0 {
                res += equation.0;
                break;
            }
        }
    }

    //println!("{:?}", eq_forms);

    res
}