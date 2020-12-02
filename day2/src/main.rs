use regex::Regex;
use std::fs;

fn main() {
    let mut validpasswords = 0;
    let re = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();

    for s in fs::read_to_string("day2input")
        .expect("filereading fucked up")
        .split("\n")
    {
        let caps = re.captures(s).unwrap();
        let llimit: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let ulimit: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let c = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = caps.get(4).unwrap().as_str();

        let count = pass.chars().filter(|x| x == &c).count();

        if count >= llimit as usize && ulimit as usize >= count {
            validpasswords = validpasswords + 1;
        }
    }
    println!("part 1\nvalid passwords count: {}", validpasswords);
}
