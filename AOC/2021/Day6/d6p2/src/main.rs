use shared;
// looping and storing vec is too slow/big (stack overflow)
fn main() {
    //for mut i in 1..14 {
    //    println!("{}", f.num_offspring_in_days(&mut i));
    //}
    //this would work but its very very very slow, can use array with positional args ([num fish
    //with 0 days until repro, num fish with 1 day until repro, ...., num fish with 8 days until
    //repro])

    // this is the pretty and fast way
    let mut days_until_fish_reproduce = initialize_until_fish_reproduce_arr(parse_lines("./input.txt"));
    // rotate array and add for new fish
    let num_days = 256;
    for _ in 0..num_days {
        let num_fresh_fishies = days_until_fish_reproduce[0];
        for dtc_ind in 1..days_until_fish_reproduce.len() {
           days_until_fish_reproduce[dtc_ind - 1] = days_until_fish_reproduce[dtc_ind];
        }
        days_until_fish_reproduce[8] = num_fresh_fishies;
        days_until_fish_reproduce[6] += num_fresh_fishies;
    }
    println!("Number of lanternfishies after {} days is: {}", num_days, days_until_fish_reproduce.iter().sum::<u64>());
}

fn parse_lines(filename: &str) -> Vec<i32> {
    let lines = shared::get_lines(filename);
    lines[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn initialize_until_fish_reproduce_arr(fish_countdowns: Vec<i32>) -> [u64; 9] {
    let mut days_until_fish_reproduce = [0; 9];
    for countdown in fish_countdowns {
        days_until_fish_reproduce[countdown as usize] += 1;   
    }
    days_until_fish_reproduce
}


