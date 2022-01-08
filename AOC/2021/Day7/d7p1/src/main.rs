use shared;
use std::collections::HashMap;

fn main() {
    let nums = parse_input("./input.txt");
    let (pos, fuel) = brute_force(nums);
    println!("The optimal position is: {}, costing {} fuel", pos, fuel);
}

fn mode(numbers: Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn parse_input(filename: &str) -> Vec<i32> {
    let lines = shared::get_lines(filename);
    lines[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn brute_force(nums: Vec<i32>) -> (i32, i32) {
    let nums_clone = nums.clone();
    let max_option = nums_clone.iter().max();
    let max = match max_option {
        Some(max) => max,
        None      => &0,
    };   
    let mut min_sum = 0;
    let mut min_num = 0;
    for i in &nums {
        min_sum += i;
    }
    for i in 0..*max + 1 {
        let mut sum = 0;
        for num in &nums {
            sum += (i - num).abs();
            //println!("{} - {} =  {}          {}: {}", i, num, (i - num).abs(), min_num, min_sum);
        }
        if sum < min_sum {
            min_num = i;
            min_sum = sum;
        }
    }
    (min_num, min_sum)
}
