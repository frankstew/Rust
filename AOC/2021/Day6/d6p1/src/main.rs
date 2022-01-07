use shared;

fn main() {
    let mut fish_cycles = parse_lines("./input.txt");
    let num_days = 80;
    model_over_days(num_days, &mut fish_cycles);
   println!("number of laternfish after {} days: {}", num_days, fish_cycles.len()); 
}

fn reproduce_laternfish(fish_cycles: &mut Vec<i32>) {
    for life_index in 0..fish_cycles.len() {
        if (fish_cycles[life_index] == 0) {
            fish_cycles[life_index] = 6;
            fish_cycles.push(8);
            continue;
        }
        fish_cycles[life_index] -= 1;
    }
}

fn model_over_days(num_days: i32, fish_cycles: &mut Vec<i32>) {
    for day in 0..num_days {
        //shared::print_vec(fish_cycles);
        reproduce_laternfish(fish_cycles);
    }
}

fn parse_lines(filename: &str) -> Vec<i32> {
    let lines = shared::get_lines(filename);
    lines[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}


