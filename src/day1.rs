use std::fs;

pub fn list_dist() -> i32 {
    let mut alpha: Vec<i32> = Vec::new();
    let mut omega: Vec<i32> = Vec::new();
    let mut res = 0;

    let file = fs::read_to_string("./data/day1/input.txt");
    //let print_file = file.expect("");
    //println!("{print_file}");

    for line in file.unwrap().lines() {
        let info = line.to_string();

        let pair = info.split("   ").collect::<Vec<&str>>();
        //println!("{:?}", pair);
        alpha.push(pair[0].parse().unwrap());
        omega.push(pair[1].parse().unwrap());
    }

    alpha.sort();
    omega.sort();

    for i in 0..alpha.len() {
        /*let x = (alpha[i] - omega[i]).abs();
        println!("here lies {x}");*/
        res += (alpha[i] - omega[i]).abs();
    }

    res

    /*println!("{:?}", alpha);
    println!("{:?}", omega);
    println!("{res}");*/
}