use std::fs;

pub fn run_day_3() {

    let input = fs::read_to_string("./inputs/day3.txt").unwrap();
    let lines: Vec<_> = input.split("\r\n").collect();

    let mut total_points = 0; 

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let half_length = chars.len() / 2;
        
        let comps = chars.split_at(half_length);
        let first = comps.0;
        let second = comps.1;
        
        let mut doubles: Vec<&char> = Vec::new();

        for c in first {
            if second.contains(&c) && !doubles.contains(&c) {
                println!("Char: {:?}", c);
                doubles.push(c);
            } 
        } 

        for d in doubles {
            total_points += get_points(*d);
        }
    } 

    println!("Total points: {:?}", total_points);
}

fn get_points(c: char) -> u32 {
    let mut points = 0;
    if c.is_uppercase() {
        points += (c as u32) - 64 + 26;
    } else if c.is_lowercase() {
        points += (c as u32) - 96;
    }
    println!("DEBUG: {:?}", points);
    points
}