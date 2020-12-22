use std::fs;

fn main() {
    let mut highest_id = 0;
    let mut lowest_id = 127 * 8 + 7;
    let mut vec = Vec::new();
    fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .for_each(|line| {
            // in input, essentially B is 1, F is 0 for rows
            // and for columns: R is 1, L is 0

            let rowstring = line.get(0..7).unwrap().replace('B', "1").replace('F', "0");
            let row =
                u32::from_str_radix(&rowstring, 2).expect(&format!("failed parsing {}", rowstring));

            let columnstring = line.get(7..).unwrap().replace('R', "1").replace('L', "0");
            let column = u32::from_str_radix(&columnstring, 2).unwrap();

            // println!("row {} column {}", rowstring, columnstring);

            let id = row * 8 + column;
            if id > highest_id {
                highest_id = id;
            }
            if id < lowest_id {
                lowest_id = id;
            }

            vec.push(id);
        });
    for i in lowest_id..highest_id {
        if vec.iter().find(|&&t| t == i).is_none() {
            println!("empty id: {}", i);
        }
    }
    println!("highest id: {}", highest_id);
}
