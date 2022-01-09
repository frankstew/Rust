fn main() {

    let x = parse_full_input("./input2.txt");
    let a = vec![String::from("acedgfb"), "cdfbe".to_string(), "gcdfa".to_string(), "fbcad".to_string(), "dab".to_string(), "cefabd".to_string(), "cdfgeb".to_string(), "eafb".to_string(), "cagedb".to_string(), "ab".to_string()];
    println!("{:?}", a);
    let y = find_num_pos(a.clone());
    println!("{:?}", y);
    //for signal in &x {
    //    for pattern in &signal[0] {
    //        //decode_wire_mapping(pattern);
    //        print!("{} ", pattern);
    //    }
    //    print!(" | ");
    //    for output in &signal[1] {
    //        print!("{} ", output);
    //    }
    //    println!("");
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

fn find_num_pos(pattern: Vec<String>) -> [i32; 10] {
    let mut num_pos = [0; 10];
    find_1_4_7_8(&pattern, &mut num_pos);
    //could try this, would be more readable
    //find_three(&pattern, &mut num_pos);
    //find_five(&pattern, &mut num_pos);
    //find_two(&pattern, &mut num_pos);
    //find_six(&pattern, &mut num_pos);
    //couls also try hashmaps, would have to loop over it but I think thats fine with iter(), its
    //2am and my brain is fried but this function finally works
    let mut p_ind = 0;
    let mut num_found = vec![false, true, false, false, true, false, false, true, true, false];
    while true {
        if is_three(pattern[p_ind].clone(), pattern[num_pos[1] as usize].clone()) {
            num_pos[3] = p_ind as i32;
            num_found[3] = true;
            p_ind = (p_ind + 1) % pattern.len();
            continue;
        } else if is_nine(pattern[p_ind].clone(), pattern[num_pos[3] as usize].clone(), pattern[num_pos[4] as usize].clone()) {
            num_pos[9] = p_ind as i32;
            num_found[9] = true;
            p_ind = (p_ind + 1) % pattern.len();
            continue;
        } else if is_five(pattern[p_ind].clone(), pattern[num_pos[9] as usize].clone(), pattern[num_pos[3] as usize].clone()) {
            num_pos[5] = p_ind as i32;
            num_found[5] = true;
            p_ind = (p_ind + 1) % pattern.len();
            continue;
        } else if is_two(pattern[p_ind].clone(), pattern[num_pos[5] as usize].clone(), pattern[num_pos[3] as usize].clone()) {
            num_pos[2] = p_ind as i32;
            num_found[2] = true;
            p_ind = (p_ind + 1) % pattern.len();
            continue;
        } else if is_six(pattern[p_ind].clone(), pattern[num_pos[5] as usize].clone(), pattern[num_pos[9] as usize].clone()) {
            num_pos[6] = p_ind as i32;
            num_found[6] = true;
            p_ind = (p_ind + 1) % pattern.len();
            continue;
        } else {
            if (count_true(&num_found) == 9) {
                set_zero(&mut num_pos);
                break;
            }
        }
        p_ind = (p_ind + 1) % pattern.len();
    }
    num_pos

}

fn set_zero(num_pos: &mut [i32]) {
    let mut found_nums = [false; 10];
    for i in 1..num_pos.len() {
        found_nums[num_pos[i] as usize] = true;
    }
    num_pos[0] = found_nums.iter().position(|&r| r == false).unwrap() as i32; 
}

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
fn is_three(s: String, one: String) -> bool {
    return s.len() == 5 && contains(s, one);
}

    // (contains 4 and contains 3 and len = 6) = 9
fn is_nine(s: String, three: String, four: String) -> bool {
    return s.len() == 6 && contains(s.clone(), three) && contains(s, four); 
}

    // (len = 5 and 9 contains and is not 3) = 5
fn is_five(s: String, nine: String, three: String) -> bool {
    return s.len() == 5 && contains(nine, s.clone()) && !contains(s, three);
}

    // (len = 5 and is not 5 or 3) = 2
fn is_two(s: String, five: String, three: String) -> bool {
    return s.len() == 5 && !contains(s.clone(), five) && !contains(s, three);
}

    // (len = 6 and contains 5, is not 9) = 6
fn is_six(s: String, five: String, nine: String) -> bool {
    return s.len() == 6 && contains(s.clone(), five) && !contains(s, nine); 
}

fn contains(s1: String, s2: String) -> bool {
    let mut res = true;
    for s in s2.chars() {
        res = s1.contains(s);
        if !res {
            return res;
        }
    }
    res
}

fn find_1_4_7_8(pattern: &Vec<String>, num_pos: &mut [i32]) {
    for p_ind in 0..pattern.len() {
        if pattern[p_ind].len() == 2 {
            num_pos[1] = p_ind as i32;
        } else if pattern[p_ind].len() == 4 {
            num_pos[4] = p_ind as i32;
        } else if pattern[p_ind].len() == 3 {
            num_pos[7] = p_ind as i32;
        } else if pattern[p_ind].len() == 7 {
            num_pos[8] = p_ind as i32;
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
