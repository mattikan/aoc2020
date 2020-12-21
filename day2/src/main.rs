use regex::Regex;
use std::fs;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let re = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();

    for s in fs::read_to_string("day2input")
        .expect("filereading fucked up")
        .split("\n")
    {
        let caps = re.captures(s).unwrap();
        let llimit: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let ulimit: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let c = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = caps.get(4).unwrap().as_str();

        let count = pass.chars().filter(|x| x == &c).count();

        if count >= llimit && ulimit >= count {
            part1 = part1 + 1;
        }

        let first = pass.chars().nth(llimit - 1).unwrap() == c;
        let second = pass.chars().nth(ulimit - 1).unwrap() == c;

        if first != second {
            part2 = part2 + 1;
        }
    }
    println!("part 1\nvalid passwords count: {}", part1);

    println!("part 2\nvalid passwords count: {}", part2);
}
