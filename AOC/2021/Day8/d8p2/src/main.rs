use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let x = parse_full_input("./input.txt");
    let mut sum = 0; 
    for signal in &x {
        for i in &signal[0] {
            print!("{} ", i);
        }
        print!("|");
        for i in &signal[1] {
            print!(" {}", i);
        }
        let num_pos = find_num_pos(&signal[0]);
        let mut num = Vec::new();
        for output in &signal[1] {
            num.push(which_num(output, &signal[0], &num_pos).ok_or("which_num failed")?);
        }
        let i = merge_digits(&num);
        println!(": {}", i);
        sum += i; 
    }
    println!("--------------------------------------------------------------------------------------------------");
    println!("Sum of line outputs: {}", sum);
    return Ok(());
}

fn merge_digits(num_list: &Vec<i32>) -> i32 {
    let base: i32 = 10;
    let mut sum = 0;
    for i in (0..num_list.len()).rev() {
        sum += base.pow((num_list.len() - i - 1).try_into().unwrap()) * num_list[i];
    }
    sum
}

//[[[in1], [out1]], [[in1], [ouy2]], ...]
fn parse_full_input(filename: &str) -> Vec<Vec<Vec<String>>> {
    let lines = shared::get_lines(filename);
    let mut res = Vec::new();
    for line in lines {
        let signal = line.split("|").map(|s| s.trim()).map(|s| s.trim_end()).map(|s| s.to_owned()).collect::<Vec<String>>();
        let parsed_input = signal[0].split(" ").map(|s| s.to_owned()).collect::<Vec<String>>(); 
        let parsed_output = signal[1].split(" ").map(|s| s.to_owned()).collect::<Vec<String>>(); 
        res.push(vec![parsed_input, parsed_output]);
    }
    res
}

fn which_num(s: &String, input: &Vec<String>, num_pos: &HashMap<i32, usize>) -> Option<i32> {
    let zero = &input[num_pos.get(&0)?.to_owned()];
    let one = &input[num_pos.get(&1)?.to_owned()];
    let two = &input[num_pos.get(&2)?.to_owned()];
    let three = &input[num_pos.get(&3)?.to_owned()];
    let four = &input[num_pos.get(&4)?.to_owned()];
    let five = &input[num_pos.get(&5)?.to_owned()];
    let six = &input[num_pos.get(&6)?.to_owned()];
    let seven = &input[num_pos.get(&7)?.to_owned()];
    let eight = &input[num_pos.get(&8)?.to_owned()];
    let nine = &input[num_pos.get(&9)?.to_owned()];
    match s {
        s if (contains(s, zero) && contains(zero, s)) => return Some(0),
        s if contains(s, one) && contains(one, s) => return Some(1),
        s if contains(s, two) && contains(two, s) => return Some(2),
        s if contains(s, three) && contains(three, s) => return Some(3),
        s if contains(s, four) && contains(four, s) => return Some(4),
        s if contains(s, five) && contains(five, s) => return Some(5),
        s if contains(s, six) && contains(six, s) => return Some(6),
        s if contains(s, seven) && contains(seven, s) => return Some(7),
        s if contains(s, eight) && contains(eight, s) => return Some(8),
        s if contains(s, nine) && contains(nine, s) => return Some(9),
        _ => return None 
    }
}

fn find_num_pos(pattern: &Vec<String>) -> HashMap<i32, usize>{
    let mut num_pos = HashMap::new();
    find_1_4_7_8(&pattern, &mut num_pos);
    find_three(&pattern, &mut num_pos);
    find_nine(&pattern, &mut num_pos);
    find_five(&pattern, &mut num_pos);
    find_two(&pattern, &mut num_pos);
    find_six(&pattern, &mut num_pos);
    find_zero(&mut num_pos);
    num_pos

}

fn find_three(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let one = &pattern[num_pos.get(&1)?.to_owned()];
    for index in 0..pattern.len() {
        if is_three(&pattern[index], one) {
            num_pos.insert(3, index);
            return Some(());
        }
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

fn find_nine(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let three = &pattern[num_pos.get(&3)?.to_owned()];
    let four = &pattern[num_pos.get(&4)?.to_owned()];
    for index in 0..pattern.len() {
        if is_nine(&pattern[index], three, four) {
            num_pos.insert(9, index);
            return Some(());
        }
    }
    println!("Couldn't find 9 in {:?}", pattern);
    return None;
}

fn find_five(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let three = &pattern[num_pos.get(&3)?.to_owned()];
    let nine = &pattern[num_pos.get(&9)?.to_owned()];
    for index in 0..pattern.len() {
        if is_five(&pattern[index], three, nine) {
            num_pos.insert(5, index);
            return Some(());
        }
    }
    println!("Couldn't find 5 in {:?}", pattern);
    return None;
}

fn find_two(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let three = &pattern[num_pos.get(&3)?.to_owned()];
    let five = &pattern[num_pos.get(&5)?.to_owned()];
    for index in 0..pattern.len() {
        if is_two(&pattern[index], three, five) {
            num_pos.insert(2, index);
            return Some(());
        }
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

fn find_six(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let five = &pattern[num_pos.get(&5)?.to_owned()];
    let nine = &pattern[num_pos.get(&9)?.to_owned()];
    for index in 0..pattern.len() {
        if is_six(&pattern[index], five, nine) {
            num_pos.insert(6, index);
            return Some(());
        }
    }
    return None;
}

fn find_zero(num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()>{
    let sum: i32 = (0..10).sum();
    let mut sum2 = 0;
    for (_key, value) in &*num_pos {
        sum2 += value.to_owned() as i32;
    }
    num_pos.insert(0, (sum - sum2) as usize);
    return Some(());
}


    // (len = 5 and contains 1) = 3
fn is_three(s: &String, one: &String) -> bool {
    return s.len() == 5 && contains(s, one);
}

    // (contains 4 and contains 3 and len = 6) = 9
fn is_nine(s: &String, three: &String, four: &String) -> bool {
    return s.len() == 6 && contains(s, three) && contains(s, four); 
}

    // (len = 5 and 9 contains and is not 3) = 5
fn is_five(s: &String, three: &String, nine: &String) -> bool {
    return s.len() == 5 && contains(nine, s) && !contains(s, three);
}

    // (len = 5 and is not 5 or 3) = 2
fn is_two(s: &String, three: &String, five: &String) -> bool {
    return s.len() == 5 && !contains(s, five) && !contains(s, three);
}

    // (len = 6 and contains 5, is not 9) = 6
fn is_six(s: &String, five: &String, nine: &String) -> bool {
    return s.len() == 6 && contains(s, five) && !contains(s, nine); 
}

fn contains(s1: &String, s2: &String) -> bool {
    for s in s2.chars() {
        if !s1.contains(s) {
            return false;
        }
    }
    true
}

fn find_1_4_7_8(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) {
    for p_ind in 0..pattern.len() {
        if pattern[p_ind].len() == 2 {
            num_pos.insert(1, p_ind);
        } else if pattern[p_ind].len() == 4 {
            num_pos.insert(4, p_ind);
        } else if pattern[p_ind].len() == 3 {
            num_pos.insert(7, p_ind);
        } else if pattern[p_ind].len() == 7 {
            num_pos.insert(8, p_ind);
        }
    }
}
