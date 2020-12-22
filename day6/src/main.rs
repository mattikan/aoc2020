use std::fs;
fn main() {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let mut sum = 0;
    let mut sum2 = 0;
    fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .for_each(|group| {
            chars.chars().for_each(|c| {
                if group.contains(c) {
                    sum = sum + 1;
                }
            });
            chars.chars().for_each(|c| {
                if group.trim().split('\n').find(|line| !line.contains(c)).is_none() {
                    sum2 = sum2 + 1;
                }
            })
            
        });
    println!("sum: {}, sum2: {}", sum, sum2);
}
