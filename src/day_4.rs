use std::fs;

use crate::Part;

pub fn run_day_4(part: Part) {
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    let lines: Vec<_> = input.split("\r\n").collect();

    let mut points = 0;

    for line in lines {
        let sections: Vec<_> = line.split(",").collect();
        //println!("{:?}", sections.len());
        let section_1: Vec<_> = sections[0].split("-").collect();
        let section_2: Vec<_> = sections[1].split("-").collect();

        let begin_1: i32 = section_1[0].parse().unwrap();
        let end_1: i32 = section_1[1].parse().unwrap();

        let begin_2: i32 = section_2[0].parse().unwrap();
        let end_2: i32 = section_2[1].parse().unwrap();

        match part {
            Part::Part1 => {
                if (begin_1 <= begin_2 && end_1 >= end_2) ||
                 (begin_2 <= begin_1 && end_2 >= end_1) {
                    points += 1;
                }
            }
            Part::Part2 => {
                if (end_1 >= begin_2 && begin_1 <= end_2) ||
                    (end_2 >= begin_1 && begin_2 <= end_1) {
                        points += 1;
                    }
            }
        }
    }

    println!("Total points: {:?}", points);
}