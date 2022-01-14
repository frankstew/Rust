use shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let incomplete_lines: Vec<String> = parse_input("./input2.txt").into_iter().filter(|line| is_valid_bool(&line)).collect();
    println!("{}", incomplete_lines.len());
    for line in incomplete_lines {
        let ending = complete_line(&line);
        // reverse line ending to get actual end
        print!("{:?}        ", line);
        println!("{:?}", ending);
    }

//    let mut score = 0;
//    for l in lines {
//        //println!("{}: {}", is_valid(&l).ok_or("Isk")?, &l);
//        let val = is_valid(&l).ok_or("idfk")?;
//        if !val.0 {
//            score += calc_score(&val.1); 
//        }
//    }
//    println!("score is: {}", score);
    Ok(())
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
                println!("Invalid closing character, couldn't find matching delimiter for '{}'", c);
                return Some((false, c));
            }
            let last_open = stack.pop()?;
            if !is_matching(&last_open, &c) {
                // invalid line
                println!("Expected '{}', found '{}'", get_matching(&last_open), c);
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

fn calc_score(c: &char) -> i32 {
    match c {
        &')' => return 3,
        &']' => return 57,
        &'}' => return 1197,
        &'>' => return 25137,
        _ => return 0
    }
}
