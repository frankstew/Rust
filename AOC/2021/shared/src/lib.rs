use std::fs;

pub fn print_stuff() {
    println!("Stuff");
}

pub fn get_int_lines(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename.to_string())
        .expect("Woops");
    // filter to remove empty lines (notr perfect)
    contents.split("\n").filter(|&elem| !is_empty_or_newline(elem)).map(|s| s.trim_end().parse::<i32>().unwrap()).collect()

    //OR 

    //let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    // + filter
    //lines
}

pub fn get_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename.to_string()).expect("Woops");
    contents.split("\n").filter(|&elem| !is_empty_or_newline(elem)).map(|s| s.trim_end().to_string()).collect()
}

pub fn is_empty_or_newline(elem: &str) -> bool {
    elem == "" || elem == "\n" || elem == "\r\n" || elem.as_bytes() == &[13] || elem.to_string().is_empty()
}


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
