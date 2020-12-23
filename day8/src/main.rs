use std::{collections::HashMap, fs};

fn main() {
    let mut lines: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| String::from(s))
        .collect();

    check(&lines);
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap().to_string();
        if line.contains("nop") {
            lines[i] = line.replace("nop", "jmp").to_string();
            if check(&lines) {
                return;
            }
            lines[i] = line.replace("jmp", "nop").to_string();
        }
        if line.contains("jmp") {
            lines[i] = line.replace("jmp", "nop").to_string();
            if check(&lines) {
                return;
            }
            lines[i] = line.replace("nop", "jmp").to_string();
        }
    }
}

fn check(lines: &Vec<String>) -> bool {
    let mut seen: HashMap<i32, bool> = HashMap::new();
    let mut acc = 0;
    let mut line = 0;
    loop {
        if !seen.get(&line).is_none() {
            println!("we've seen line {} before, acc is {}", &line, &acc);
            return false;
        }
        seen.insert(line, true);
        let command = lines.get(line as usize);
        if command.is_none() {
            println!("reached the end! acc is {}", acc);
            return true;
        }

        let command = command.unwrap();
        if command.contains("nop") {
            line = line + 1;
            continue;
        } else if command.contains("acc") {
            acc = acc + command.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            line = line + 1;
        } else if command.contains("jmp") {
            line = line + command.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        }
    }
}
