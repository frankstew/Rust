fn main() {

    let x = parse_full_input("./input2.txt");
    let mut num_pos = [0; 10];
    find_1_4_7_8(&x[0][0], &mut num_pos);
    println!("{:?}", num_pos);
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
        let signal = line.split("|").map(|s| s.to_owned()).collect::<Vec<String>>();
        let parsed_input = signal[0].split(" ").map(|s| s.to_owned()).collect::<Vec<String>>(); 
        let parsed_output = signal[1].split(" ").map(|s| s.to_owned()).collect::<Vec<String>>(); 
        res.push(vec![parsed_input, parsed_output]);
    }
    res
}

fn find_num_pos(pattern: Vec<String>) -> [i32; 10] {
    // 1, 4, 7, 8 given
    // (leftover) = 0
    let mut num_pos = [0; 10];
    find_1_4_7_8(&pattern, &mut num_pos);
    for p_ind in 0..pattern.len() {
        if is_three(pattern[p_ind].clone(), pattern[num_pos[1] as usize].clone()) {
            num_pos[3] = p_ind as i32;
        } else if is_nine(pattern[p_ind].clone(), pattern[num_pos[3] as usize].clone(), pattern[num_pos[4] as usize].clone()) {
            num_pos[9] = p_ind as i32;
        } else if is_five(pattern[p_ind].clone(), pattern[num_pos[9] as usize].clone(), pattern[num_pos[3] as usize].clone()) {
            num_pos[5] = p_ind as i32;
        } else if is_two(pattern[p_ind].clone(), pattern[num_pos[5] as usize].clone(), pattern[num_pos[3] as usize].clone()) {
            num_pos[2] = p_ind as i32;
        } else if is_six(pattern[p_ind].clone(), pattern[num_pos[5] as usize].clone(), pattern[num_pos[9] as usize].clone()) {
            num_pos[6] = p_ind as i32;
        } else {
            num_pos[0] = p_ind as i32;
        }
    }
    num_pos

}

    // (len = 5 and contains 1) = 3
fn is_three(s: String, one: String) -> bool {
    return s.len() == 5 && contains(s, one);
}

    // (contains 4 and contains 3 and len = 6) = 9
fn is_nine(s: String, three: String, four: String) -> bool {
    return s.len() == 6 && contains(s.clone(), three) && contains(s, four); 
}

    // (len = 5 and 9 contains) = 5
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
            num_pos[4] == p_ind as i32;
        } else if pattern[p_ind].len() == 3 {
            num_pos[7] == p_ind as i32;
        } else if pattern[p_ind].len() == 7 {
            num_pos[8] == p_ind as i32;
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
