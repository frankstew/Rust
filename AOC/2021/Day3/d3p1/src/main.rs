use shared;



fn main() {
    let binary_nums = shared::get_lines("./input.txt");
    let mut binary_gamma_rate = "".to_string();
    let mut binary_epsilon_rate = "".to_string();
    let mut column_sums = [0; 12];
    for bnum in binary_nums.iter() {
        let mut column_sums_ind = 0;
        for chr in bnum.chars() {
            if (chr == '1') {
               column_sums[column_sums_ind] += 1; 
            }
            column_sums_ind += 1;
        }
    }
    for dig in column_sums.iter() {
        if (dig > &(binary_nums.len() / 2)) {
            binary_gamma_rate.push('1');
            binary_epsilon_rate.push('0');
        } else {
            binary_gamma_rate.push('0');
            binary_epsilon_rate.push('1');
        }
    }
    println!("{}, {}", binary_gamma_rate, binary_epsilon_rate);

    println!("Power consumption: {}", isize::from_str_radix(&binary_gamma_rate, 2).unwrap() * isize::from_str_radix(&binary_epsilon_rate, 2).unwrap());

}


