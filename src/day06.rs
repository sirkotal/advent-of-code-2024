use std::fs;

#[derive(Debug)]
struct GuardCoords {
    y: usize,
    x: usize,
}

pub fn guard_map() -> i32 {
    let mut data: Vec<String> = Vec::new();
    let mut guard_index = GuardCoords {
        y: 0,
        x: 0,
    };
    let mut direction: &str = "up";
    let mut res: i32 = 0;
    let mut bounds: bool = true;

    let file = fs::read_to_string("./data/day06/input.txt");

    for line in file.unwrap().lines() {
        let info = line.to_string();
        data.push(info);
    }

    for i in 0..(data.len() - 1) {
        if data[i].contains("^") {
            let index = data[i].find("^").unwrap();

            guard_index.x = index;
            guard_index.y = i;
        }
    }

    println!("{:?}", data.len());
    println!("{:?}", guard_index);

    while bounds {
        match direction {
            "up" => {
                if (guard_index.y as i32) - 1 < 0 {
                    bounds = false;
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();
                    break;
                }
                if data[guard_index.y - 1].chars().collect::<Vec<char>>()[guard_index.x] == '#' {
                    direction = "right";
                }
                else {
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();

                    guard_index.y -= 1;

                    collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = '^';
                    data[guard_index.y] = collection.into_iter().collect();
                    println!("{:?}", guard_index);
                }
            }
            "right" => {
                if (guard_index.x as i32) + 1 > ((data[0].len() - 1) as i32) {
                    bounds = false;
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();
                    break;
                }
                if data[guard_index.y].chars().collect::<Vec<char>>()[guard_index.x + 1] == '#' {
                    direction = "down";
                }
                else {
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();
                    
                    guard_index.x += 1;

                    collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = '^';
                    data[guard_index.y] = collection.into_iter().collect();
                    println!("{:?}", guard_index);
                }
            }
            "down" => {
                if (guard_index.y as i32) + 1 > ((data.len() - 1) as i32) {
                    bounds = false;
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();
                    break;
                }
                if data[guard_index.y + 1].chars().collect::<Vec<char>>()[guard_index.x] == '#' {
                    direction = "left";
                }
                else {
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();

                    guard_index.y += 1;

                    collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = '^';
                    data[guard_index.y] = collection.into_iter().collect();
                    println!("{:?}", guard_index);
                }
            }
            "left" => {
                if (guard_index.x as i32) - 1 < 0 {
                    bounds = false;
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();
                    break;
                }
                if data[guard_index.y].chars().collect::<Vec<char>>()[guard_index.x - 1] == '#' {
                    direction = "up";
                }
                else {
                    let mut collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = 'X';
                    data[guard_index.y] = collection.into_iter().collect();

                    guard_index.x -= 1;

                    collection = data[guard_index.y].chars().collect::<Vec<char>>();
                    collection[guard_index.x] = '^';
                    data[guard_index.y] = collection.into_iter().collect();
                    println!("{:?}", guard_index);
                }
            }
            _ => {
                println!("Something went terribly wrong...");
            }
        }
    }

    //println!("{:?}", data);
    for row in data {
        res += row.chars().filter(|c| *c == 'X').count() as i32;
    }

    res
}