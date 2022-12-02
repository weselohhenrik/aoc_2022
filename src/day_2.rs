use std::fs;

use crate::Part;

pub fn run_day_2(part: Part) {
    let input = fs::read_to_string("./inputs/day2.txt").unwrap();
    let lines: Vec<_> = input.split("\r\n").collect();

    let mut total_points = 0;

    for line in lines {
        let chars: Vec<_> = line.split(" ").collect();
        match part {
            Part::Part1 => {
                total_points += evaluate_game(chars[0], chars[1]);
            }
            Part::Part2 => {
                total_points += evaluate_game_part_2(chars[0], chars[1])
            }
        }
        //println!("Debug points: {:?}", total_points);
    }
    println!("Total points: {:?}", total_points);
}

fn evaluate_game_part_2(op_move: &str, result: &str)-> i32 {
    let mut points = 0;
    match result {
        "X" => {

            match op_move {
                "A" => {
                    points += evaluate_game(op_move, "Z");
                }
                "B" => {
                    points += evaluate_game(op_move, "X");
                }
                "C" => {
                    points += evaluate_game(op_move, "Y");
                }
                _ => {
                    unreachable!()
                }
            }
        }
        "Y" => {
            match op_move {
                "A" => {
                    points += evaluate_game(op_move, "X");
                }
                "B" => {
                    points += evaluate_game(op_move, "Y");
                }
                "C" => {
                    points += evaluate_game(op_move, "Z");
                }
                _ => {
                    unreachable!()
                }
            }
        }
        "Z" => {
            
            match op_move {
                "A" => {
                    points += evaluate_game(op_move, "Y");
                }
                "B" => {
                    points += evaluate_game(op_move, "Z");
                }
                "C" => {
                    points += evaluate_game(op_move, "X");
                }
                _ => {
                    unreachable!()
                }
            }
        }
        _ => {
            unreachable!()
        }
    }

    points
}

fn evaluate_game(op_move: &str, my_move: &str)-> i32 {
    let mut points = 0;

    match op_move {
        "A" => {
            match my_move {
                "X" => {
                    points+= 1; // for rock
                    points += 3; // for draw
                }
                "Y" => {
                    points += 2; // for paper
                    points += 6; // for victory
                }
                "Z" => {
                    points += 3; // for scissor; 0 for loss
                }
                _ => {
                   println!("Invalid move: {:?}", my_move); 
                }
            }
        }
        "B" => {
            match my_move {
                "X" => {
                    points += 1; // for rock; 0 for loss
                }
                "Y" => {
                    points += 2; // 2 for paper
                    points += 3; // 3 for draw
                }
                "Z" => {
                    points += 3; // 3 for scissor
                    points += 6; // 6 for victory
                }
                _ => {
                    println!("Invalid move: {:?}", my_move);
                }
            }
        }
        "C" => {
            match my_move {
                "X" => {
                    points += 1; // 1 for rock
                    points += 6; // 6 for victory
                }
                "Y" => {
                    points += 2; // 2 for paper; 0 for loss
                
                }
                "Z" => {
                    points += 3; // 3 for scissor
                    points += 3; // 3 for draw
                }
                _ => {
                    println!("Invalid move: {:?}", my_move);
                }
            }
        }
        _ => {
            println!("Invalid move: {:?}", op_move);
        }
    }

    points
}