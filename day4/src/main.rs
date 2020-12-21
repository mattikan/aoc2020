use regex::Regex;
use std::fs;

fn main() {
    let mut valids = 0;
    fs::read_to_string("input")
        .expect("reading input failed")
        .trim()
        .split("\n\n")
        .for_each(|passport| {
            if filters(passport) {
                valids = valids + 1;
            }
        });
    println!("valid passports: {}", valids);
}

fn filters(passport: &str) -> bool {
    return contains_checks(passport)
        && check(passport, Regex::new(r"byr:(19[2-9]\d|200[012])(\s|$)").unwrap())
        && check(passport, Regex::new(r"iyr:20(1\d|20)(\s|$)").unwrap())
        && check(passport, Regex::new(r"eyr:20(2\d|30)(\s|$)").unwrap())
        && check(
            passport,
            Regex::new(r"hgt:(1([5-8]\d|9[0123])cm|(59|6\d|7[01-6])in)(\s|$)").unwrap(),
        )
        && check(passport, Regex::new(r"hcl:#[0-9a-f]{6}(\s|$)").unwrap())
        && check(
            passport,
            Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)(\s|$)").unwrap(),
        )
        && check(passport, Regex::new(r"pid:(\d){9}(\s|$)").unwrap());
}

fn contains_checks(passport: &str) -> bool {
    return passport.contains("byr")
        && passport.contains("iyr")
        && passport.contains("eyr")
        && passport.contains("hgt")
        && passport.contains("hcl")
        && passport.contains("ecl")
        && passport.contains("pid");
}

fn check(passport: &str, field: Regex) -> bool {
    return !field.captures_iter(passport).next().is_none();
}
