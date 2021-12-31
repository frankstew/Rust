use std::fs;
use std::string::ToString;

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



pub fn print_vec<T: ToString>(v: &Vec<T>) {
    let mut vec_str = String::from("[");
    for i in v.iter() {
        let mut append_str = i.to_string();
        append_str.push_str(", ");
        vec_str.push_str(&append_str);
    }
    vec_str = vec_str[0..vec_str.len() - 2].to_string();
    vec_str.push_str("]");
    println!("{}", vec_str);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
