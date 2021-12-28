use std::fs;
use std::any::type_name;


fn main() {
    let depths = get_int_lines("./depths.txt");
    println!("Depth increases: {}", get_rolling_depth_increases(depths, 3));
    
}


fn get_rolling_depth_increases(depths: Vec<i32>, rolling_window: usize) -> i32 {
    let mut inc_cnt = 0;
    let chunked_depths: Vec<&[i32]> = depths.windows(rolling_window).collect();
    let mut chunked_depths_iter = chunked_depths.iter();
    let mut prev_chunk_depth: i32 = chunked_depths_iter.next().unwrap().to_owned().iter().sum();
    for depth_chunk in chunked_depths_iter {
        let curr_chunk_depth: i32 = depth_chunk.to_owned().iter().sum();
        if (curr_chunk_depth > prev_chunk_depth) {
            inc_cnt += 1;
        }
        prev_chunk_depth = curr_chunk_depth;
    }
    inc_cnt
}


fn get_int_lines(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename.to_string())
        .expect("Woops");
    // filter to remove empty lines (notr perfect)
    contents.split("\n").filter(|&elem| (elem != "" && elem != "\n")).map(|s| s.parse::<i32>().unwrap()).collect()

    //OR 

    //let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    // + filter
    //lines
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
