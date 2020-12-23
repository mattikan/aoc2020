use std::{collections::HashMap, fs};

fn main() {
    let lines: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| String::from(s))
        .collect();

    let mut seen: HashMap<i32, bool> = HashMap::new();
    let mut acc = 0;
    let mut line = 0;
    loop {
        if !seen.get(&line).is_none() {
            println!("we've seen line {} before, acc is {}", &line, &acc);
            break;
        }
        seen.insert(line, true);
        let command = lines.get(line as usize).unwrap();
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
