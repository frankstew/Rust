use std::fs;
use std::any::type_name;


fn main() {
    let depths = get_int_lines("./depths.txt");
    println!("Depth increases: {}", get_depth_increases(depths));
    
}

fn get_int_lines(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename.to_string())
        .expect("Woops");
    // filter to remove empty lines (notr perfect)
    contents.split("\n").filter(|&elem| !is_empty_or_newline(elem)).map(|s| s.trim_end().parse::<i32>().unwrap()).collect()

    //OR 

    //let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    // + filter
    //lines
}

fn is_empty_or_newline(elem: &str) -> bool {
    (elem == "" || elem == "\n" || elem == "\r\n" || elem.as_bytes() == &[13] || elem.to_string().is_empty())
}

fn get_depth_increases(depths: Vec<i32>) -> i32{
    let mut depths_iter = depths.iter();
    let mut inc_cnt = 0;
    let mut prev_depth = depths_iter.next().unwrap();
    for depth in depths_iter {
        if (depth > prev_depth) {
            inc_cnt += 1;
        }
        prev_depth = depth;
    }
    inc_cnt
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
