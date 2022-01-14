use shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let incomplete_lines: Vec<String> = parse_input("./input.txt").into_iter().filter(|line| is_valid_bool(&line)).collect();
    let mut scores = Vec::new();
    for line in incomplete_lines {
        let ending = complete_line(&line).ok_or("Who knows")?;
        let score = calc_score_for_ending(&ending);
        // reverse line ending to get actual end
        print!("{:?}        ", line);
        let ending_combined = combine_char_vec(&ending);
        println!("{:?}: {}", ending_combined, score);
        scores.push(score);
    }
    scores.sort();
    println!("Middle score is {}", scores[scores.len() / 2 as usize]);

    Ok(())
}

fn combine_char_vec(ending: &Vec<char>) -> String {
    let mut s = String::from("");
    for c in ending {
        s.push(*c);
    }
    s
}

fn parse_input(filename: &str) -> Vec<String> {
    shared::get_lines(filename)
}

fn complete_line(line: &String) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening_char(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }

    }
    stack.reverse();
    for i in 0..stack.len() {
        stack[i] = get_matching(&stack[i]); 
    }
    Some(stack)
    
}

fn is_valid_bool(line: &String) -> bool {
    let val = is_valid(line);
    match val {
        Some(b) => return b.0,
        _ => return false
    }
}

fn is_valid(line: &String) -> Option<(bool, char)> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening_char(&c) {
            stack.push(c);
        } else {
            if stack.len() == 0 {
                return Some((false, c));
            }
            let last_open = stack.pop()?;
            if !is_matching(&last_open, &c) {
                // invalid line
                return Some((false, c));
            }
        }
    }
    // check if line was completed
    //stack.len() == 0
    Some((true, '_'))
}

fn get_matching(c: &char) -> char {
    match c {
        &'(' => return ')',
        &'{' => return '}',
        &'[' => return ']',
        &'<' => return '>',
        _ => return '_' 
    }
}

fn is_matching(opener: &char, c: &char) -> bool {
    match opener {
        &'(' => return c == &')',
        &'{' => return c == &'}',
        &'[' => return c == &']',
        &'<' => return c == &'>',
        _ => return false
    }
}

fn is_opening_char(c: &char) -> bool {
    c == &'(' || c == &'{' || c == &'[' || c == &'<'
}

fn calc_score_for_char(c: &char) -> u64 {
    match c {
        &')' => return 1,
        &']' => return 2,
        &'}' => return 3,
        &'>' => return 4,
        _ => return 0
    }
}

fn calc_score_for_ending(ending: &Vec<char>) -> u64 {
    let mut score: u64 = 0;
    for i in ending {
        score *= 5;
        score += calc_score_for_char(&i);
    }
    score
}
