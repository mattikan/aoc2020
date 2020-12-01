use std::fs;

fn main() {
    let input = fs::read_to_string("day1input").expect("file opening fucked up");
    let numerot: Vec<i32> = input
        .split("\n")
        .map(|s| match s.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        })
        .collect();

    // numerot.iter().for_each(|i| println!("{}", i));

    numerot.iter().for_each(|z| {
        numerot.iter().for_each(|x| {
            if z + x == 2020 {
                println!("numbers found: {} and {}, solution is {}", z, x, z * x)
            }
        })
    });
}
