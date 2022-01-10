use std::collections::HashMap;
use std::error;

fn main() {

    let x = parse_full_input("./input2.txt");
    let a = vec![String::from("acedgfb"), "cdfbe".to_string(), "gcdfa".to_string(), "fbcad".to_string(), "dab".to_string(), "cefabd".to_string(), "cdfgeb".to_string(), "eafb".to_string(), "cagedb".to_string(), "ab".to_string()];
    //let b = vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")];
    //println!("{:?}", a);
    let y = find_num_pos(a.clone());
    //let z = which_num(&b[3], a.clone(), &y);
    println!("{:?}", y);
    //println!("{:?}", z);
    //let mut sum = 0; 
    //for signal in &x {
    //        let mut num_pos = [0; 10];
    //        num_pos = find_num_pos(signal[0].clone());
    //        println!("{:?}", signal[0].clone());
    //        println!("{:?}", num_pos.clone());
    //    //for pattern in &signal[0] {
    //    //    num_pos = find_num_pos(pattern.clone());
    //    //    print!("{:?} ", num_pos);
    //    //}
    //    //print!(" | ");
    //    let mut num = Vec::new();
    //    for output in &signal[1] {
    //        num.push(which_num(output, signal[0].clone(), &num_pos));
    //        //print!("{}", which_num(output, signal[0].clone(), &num_pos))
    //    }
    //    let i = merge_digits(&num);
    //    sum += i; 
    //    println!("{}", i);
    //}
}

//[[[inpu1], [out1]], [[in1], [ou2]], ...]
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

fn which_num(s: &String, input: Vec<String>, num_pos: &[i32]) -> i32 {
    let zero = &input[num_pos[0] as usize];
    let one = &input[num_pos[1] as usize];
    let two = &input[num_pos[2] as usize];
    let three = &input[num_pos[3] as usize];
    let four = &input[num_pos[4] as usize];
    let five = &input[num_pos[5] as usize];
    let six = &input[num_pos[6] as usize];
    let seven = &input[num_pos[7] as usize];
    let eight = &input[num_pos[8] as usize];
    let nine = &input[num_pos[9] as usize];
    return -1;
//    if (contains(s.clone(), zero.clone()) && contains(zero.clone(), s.clone(),)) {
//        return 0;
//    } else if contains(s.clone(), one.clone()) && contains(one.clone(), s.clone(),) {
//        return 1;
//    } else if contains(s.clone(), two.clone()) && contains(two.clone(), s.clone(),) {
//        return 2;
//    } else if contains(s.clone(), three.clone()) && contains(three.clone(), s.clone(),) {
//        return 3;
//    } else if contains(s.clone(), four.clone()) && contains(four.clone(), s.clone(),) {
//        return 4;
//    } else if contains(s.clone(), five.clone()) && contains(five.clone(), s.clone(),) {
//        return 5;
//    } else if contains(s.clone(), six.clone()) && contains(six.clone(), s.clone(),) {
//        return 6;
//    } else if contains(s.clone(), seven.clone()) && contains(seven.clone(), s.clone(),) {
//        return 7;
//    } else if contains(s.clone(), eight.clone()) && contains(eight.clone(), s.clone(),) {
//        return 8;
//    } else if contains(s.clone(), nine.clone()) && contains(nine.clone(), s.clone(),) {
//        return 9;
//    } else {
//        return -1;
//    }
}

fn find_num_pos(pattern: Vec<String>) -> HashMap<i32, usize>{
    let mut num_pos = HashMap::new();
    find_1_4_7_8(&pattern, &mut num_pos);
    //could try this, would be more readable
    find_three(&pattern, &mut num_pos);
    find_nine(&pattern, &mut num_pos);
    find_five(&pattern, &mut num_pos);
    find_two(&pattern, &mut num_pos);
    find_six(&pattern, &mut num_pos);
    find_zero(&pattern, &mut num_pos);
    //couls also try hashmaps, would have to loop over it but I think thats fine with iter(), its
    //2am and my brain is fried but this function finally works
    num_pos

}

enum GeneralError {

}

fn find_three(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let mut index = 0;
    let one = &pattern[num_pos.get(&1)?.to_owned()];
    while true {
        if is_three(&pattern[index], one) {
            num_pos.insert(3, index);
            return Some(());
        }
        index += 1;
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

fn find_nine(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    let mut index = 0;
    let three = &pattern[num_pos.get(&3)?.to_owned()];
    let four = &pattern[num_pos.get(&4)?.to_owned()];
    while true {
        if is_nine(&pattern[index], three, four) {
            num_pos.insert(9, index);
            return Some(());
        }
        index += 1;
    }
    println!("Couldn't find 9 in {:?}", pattern);
    return None;
}

fn find_five(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    return None;
    let mut index = 0;
    let one = &pattern[num_pos.get(&1)?.to_owned()];
    while true {
        if is_three(&pattern[index], one) {
            num_pos.insert(3, index);
            return Some(());
        }
        index += 1;
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

fn find_two(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    return None;
    let mut index = 0;
    let one = &pattern[num_pos.get(&1)?.to_owned()];
    while true {
        if is_three(&pattern[index], one) {
            num_pos.insert(3, index);
            return Some(());
        }
        index += 1;
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

fn find_six(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()> {
    return None;
    let mut index = 0;
    let one = &pattern[num_pos.get(&1)?.to_owned()];
    while true {
        if is_three(&pattern[index], one) {
            num_pos.insert(3, index);
            return Some(());
        }
        index += 1;
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

fn find_zero(pattern: &Vec<String>, num_pos: &mut HashMap<i32, usize>) -> core::option::Option<()>{
    return None;
    let mut index = 0;
    let one = &pattern[num_pos.get(&1)?.to_owned()];
    while true {
        if is_three(&pattern[index], one) {
            num_pos.insert(3, index);
            return Some(());
        }
        index += 1;
    }
    println!("Couldn't find 3 in {:?}", pattern);
    return None;
}

//fn set_zero(num_pos: &mut [i32]) {
//    let mut found_nums = [false; 10];
//    for i in 1..num_pos.len() {
//        found_nums[num_pos[i] as usize] = true;
//    }
//    num_pos[0] = found_nums.iter().position(|&r| r == false).unwrap() as i32; 
//}

fn count_true(vec: &Vec<bool>) -> i32 {
    let mut num_true = 0;
    for i in vec {
        if *i {
            num_true += 1;
        }
    }
    num_true
}

    // (len = 5 and contains 1) = 3
fn is_three(s: &String, one: &String) -> bool {
    return s.len() == 5 && contains(s, one);
}

    // (contains 4 and contains 3 and len = 6) = 9
fn is_nine(s: &String, three: &String, four: &String) -> bool {
    return s.len() == 6 && contains(s, three) && contains(s, four); 
}
//
//    // (len = 5 and 9 contains and is not 3) = 5
//fn is_five(s: String, nine: String, three: String) -> bool {
//    return s.len() == 5 && contains(nine, s.clone()) && !contains(s, three);
//}
//
//    // (len = 5 and is not 5 or 3) = 2
//fn is_two(s: String, five: String, three: String) -> bool {
//    return s.len() == 5 && !contains(s.clone(), five) && !contains(s, three);
//}
//
//    // (len = 6 and contains 5, is not 9) = 6
//fn is_six(s: String, five: String, nine: String) -> bool {
//    return s.len() == 6 && contains(s.clone(), five) && !contains(s, nine); 
//}

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

fn parse_input(filename: &str) -> Vec<Vec<String>> {
    let lines = shared::get_lines(filename);
    let mut res = Vec::new();
    for line in lines {
        let output = line.split("|").map(|s| s.to_owned()).collect::<Vec<String>>();
        let parsed_output = output[1].split(" ").map(|s| s.to_owned()).collect::<Vec<String>>(); 
        res.push(parsed_output);
    }
    res
}

fn count_nums_with_unique_num_segments(strs: Vec<Vec<String>>) -> i32 {
    let mut num_uniques = 0;
    for output in strs {
        for string in output {
            if string.len() == 2 || string.len() == 4 || string.len() == 3 || string.len() == 7 {
                num_uniques += 1
            }
        }
    }
    num_uniques
}
