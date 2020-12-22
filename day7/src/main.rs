use regex::Regex;
use std::fs;
fn main() {
    let mut bags = vec![String::from("shiny gold")];
    let mut rules = Vec::new();
    let re_container = Regex::new(r"^(\w+\s\w+)\s").unwrap();
    let re_contained = Regex::new(r"(\d+)\s(\w+\s\w+)\sbag").unwrap();

    fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .for_each(|row| {
            let container = re_container.captures(row).unwrap()[1].to_string();
            let contained = re_contained.captures_iter(row);

            contained.for_each(|capture| {
                rules.push((
                    container.to_string(),
                    (capture[1].to_string(), capture[2].to_string()),
                ));
            });
        });
    loop {
        let mut new_bags: Vec<String> = Vec::new();

        bags.iter().for_each(|bag| {
            rules
                .iter()
                .filter(|&rule| rule.1 .1.eq(bag))
                .filter(|&rule| bags.iter().find(|&existing| existing.eq(&rule.0)).is_none())
                .for_each(|rule| {
                    new_bags.push(rule.0.to_string());
                });
        });
        if new_bags.len() > 0 {
            new_bags.iter().for_each(|new| bags.push(new.to_string()))
        } else {
            bags.sort();
            bags.dedup();
            bags.iter().for_each(|bag| println!("{}", bag));
            println!("part 1 answer: {}", bags.len());
            break;
        }
    }

    let mut sum = 0;
    let mut bags = vec![String::from("shiny gold")];
    while let Some(bag) = bags.pop() {
        rules
            .iter()
            .filter(|rule| rule.0.eq(&bag))
            .for_each(|rule| {
                let amount = rule.1 .0.parse::<i32>().unwrap();
                sum = sum + amount;
                for _i in 0..amount {
                    bags.push(rule.1 .1.to_string());
                }
            })
    }
    println!("part 2 answer: {}", sum);
}
