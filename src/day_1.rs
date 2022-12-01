use std::fs;

use crate::Part;


pub fn run_day_1(part: Part) {
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();

    let mut max_calories = 0;

    let groups: Vec<_> = input.split("\r\n\r\n").collect();

    let mut total_calories_per_elve: Vec<i32> = Vec::new();

    for group in groups {
        let mut total_calories = 0;
        let lines: Vec<_> = group.split("\r\n").collect();

        for line in lines {
            total_calories += line.parse::<i32>().unwrap();
        }

        total_calories_per_elve.push(total_calories);
    }

    total_calories_per_elve.sort();
    total_calories_per_elve.reverse();

    match part {
        Part::Part1 => { max_calories = total_calories_per_elve[0]; },
        Part::Part2 => { 
            for i in 0..3 {
                max_calories += total_calories_per_elve[i];
                println!("Debug: {:?}", max_calories);
            }
        }
    };

    println!("{:?}", max_calories);
}