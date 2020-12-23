use std::fs;

fn main() {
    let numbers: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut first = 0;
    let mut current = 25;

    while current < numbers.len() {
        let answer = part_1_helper(&numbers, first, current);
        if !answer.0 {
            println!("answer found! {} ", answer.1);
            for i in 1..numbers.len() {
                let answer = part_2_helper(&numbers, i, answer.1);
                if answer.0 {
                    println!("part 2 answer found! {}", answer.1);
                }
            }
        }

        first += 1;
        current += 1;
    }
}

fn part_1_helper(vec: &Vec<i64>, first: usize, current: usize) -> (bool, i64) {
    let sum = vec.get(current).unwrap();
    for &i in vec.get(first..current).unwrap() {
        if i != sum / 2 && vec.iter().find(|&&j| i + j == *sum).is_some() {
            return (true, *sum);
        }
    }
    return (false, *sum);
}

fn part_2_helper(vec: &Vec<i64>, len: usize, sum: i64) -> (bool, i64) {
    for i in 0..vec.len() - len + 1 {
        if vec.get(i..i + len).unwrap().iter().sum::<i64>() == sum {
            println!("len: {}, sum: {}", len, sum);
            let min = vec.get(i..i + len).unwrap().iter().min().unwrap();
            let max = vec.get(i..i + len).unwrap().iter().max().unwrap();
            return (true, min + max);
        }
    }
    return (false, 0);
}
