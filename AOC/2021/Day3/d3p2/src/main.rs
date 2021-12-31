use shared;
use std::string::ToString;



fn main() {
    let mut binary_nums = shared::get_lines("./input.txt");

    //could also try some borrowing stuff instead of clone, but I need to modify (at least the way I'm doing it) so
    //idk if borrowing would work bc it would still get mutated 
    let oxygen_generator_rating = get_oxygen_generator_rating(binary_nums.clone());
    let co2_scrubber_rating = get_CO2_scrubber_rating(binary_nums.clone());
    let life_support_rating = isize::from_str_radix(&oxygen_generator_rating, 2).unwrap() * isize::from_str_radix(&co2_scrubber_rating, 2).unwrap();
    println!("life support rating: {}", life_support_rating);
}




fn get_most_common_digs(bnums: &Vec<String>) -> Vec<i32> {
    let mut column_sums: Vec<i32> = vec![0; bnums[0].len()];
    for bnum in bnums.iter() {
        let mut column_sums_ind = 0;
        for chr in bnum.chars() {
            if (chr == '1') {
               column_sums[column_sums_ind] += 1; 
            }
            column_sums_ind += 1;
        }
    }
    column_sums.iter().map(|n| {
        // >= b/c 1 is the default if there are equal amounts of both 
        if (*n as f32 >= bnums.len() as f32 / 2.) {
            return 1;
        } else {
            return 0;
        }
    }).collect::<Vec<i32>>()
}


fn get_oxygen_generator_rating(mut binary_nums: Vec<String>) -> String {
    let mut column_sums = get_most_common_digs(&binary_nums);
    let mut index = 0;
    while index < column_sums.len() {
        binary_nums = binary_nums.iter().filter(|&elem| elem.chars().nth(index).unwrap().to_digit(2).unwrap() == column_sums[index] as u32).map(|s| s.to_owned()).collect::<Vec<String>>();
        index += 1;
        column_sums = get_most_common_digs(&binary_nums);
        if (binary_nums.len() == 1) {
            return String::from(&binary_nums[0])
        }
    }
    String::from(&binary_nums[0])
}


fn get_CO2_scrubber_rating(mut binary_nums: Vec<String>) -> String {
    let mut column_common_dig = get_most_common_digs(&binary_nums);

    let mut index = 0;
    while index < column_common_dig.len() {
        binary_nums = binary_nums.iter().filter(|&elem| elem.chars().nth(index).unwrap().to_digit(2).unwrap() != column_common_dig[index] as u32).map(|s| s.to_owned()).collect::<Vec<String>>();
        index += 1;
        column_common_dig = get_most_common_digs(&binary_nums);
        if (binary_nums.len() == 1) {
            return String::from(&binary_nums[0])
        }
    }
    String::from(&binary_nums[0])
}
