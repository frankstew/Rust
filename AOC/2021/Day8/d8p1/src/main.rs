fn main() {
    println!("number of 1's, 4's, 7's, and 8's: {}", count_nums_with_unique_num_segments(parse_input("./input.txt")));
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
