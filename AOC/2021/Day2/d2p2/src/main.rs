use shared;
use std::collections::HashMap;
use std::fs;







fn main() {
    let direction_commands = get_instructions_lines("./input.txt");
    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;
    for command in direction_commands.iter() {
        if (&command.0 == "forward") {
            horiz += command.1;
            depth += aim * command.1;
        } else if (&command.0 == "down") {
            aim += command.1;
        } else if (&command.0 == "up") {
            aim -= command.1;
        }
    }
    println!("{} * {} = {}", horiz, depth, horiz * depth);
}


fn get_instructions_lines(filename: &str) -> Vec<(String, i32)> {
    let contents = fs::read_to_string(filename.to_string()).expect("Error readiing file: ");
    let contents_vec = contents.split("\n").filter(|&elem| !shared::is_empty_or_newline(elem)).map(|s| s.trim_end()).collect::<Vec<&str>>();
    let contents_iter = contents_vec.iter();
    let mut instructions: Vec<(String, i32)> = Vec::new();
    for line in contents_iter {
        let instruction = line.split(" ").collect::<Vec<&str>>();
        instructions.push((instruction[0].to_string(), instruction[1].parse::<i32>().unwrap()));
    }
    instructions
}

