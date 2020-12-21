use std::fs;

fn main() {
    println!("part 1 answer: {}", check_slope(1, 3));
    println!(
        "part 2 answer: {}",
        check_slope(1, 1)
            * check_slope(1, 3)
            * check_slope(1, 5)
            * check_slope(1, 7)
            * check_slope(2, 1)
    );
}

fn check_slope(down: usize, right: usize) -> usize {
    let mut row = 0;
    let mut column = 0;
    let mut hits = 0;

    let rows: Vec<String> = fs::read_to_string("input")
        .expect("reading failed")
        .trim()
        .split("\n")
        .map(String::from)
        .collect();

    while row < rows.len() {
        if rows[row].chars().nth(column).unwrap().eq(&'#') {
            hits = hits + 1;
        }
        row = row + down;
        column = column + right;
        if column >= rows[0].len() {
            column = column - rows[0].len();
        }
    }
    return hits;
}
