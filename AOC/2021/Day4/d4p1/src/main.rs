use shared;





fn main() {
    let lines = shared::get_lines("./input.txt");
    let board_lines = &lines[1..lines.len()];
    let owned_board_lines = board_lines.to_owned();
    let mut split_board_lines = Vec::new();
    for board_line in owned_board_lines.iter() {
        let i: Vec<&str> = board_line.split(" ").collect();
        shared::print_type_of(&i);
        println!("{}", i[1]);
        split_board_lines.push(i);
    }
    shared::print_type_of(&split_board_lines);
    let x: Vec<Vec<Vec<&str>>> = split_board_lines.chunks(5).map(|s| s.to_owned()).collect();
    // EMPTY b/c single digit, something about splitting
    println!("{}", x[0][0][4]);
    let calling_order = lines[0].to_owned();
}

//fn parse_bingo_input(filename: &str) -> (Vec<String>, Vec<&[String]>) {
//    let lines = shared::get_lines("./input.txt");
//    let calling_order = lines[0].to_owned().split(",").map(|s| s.to_owned()).collect::<Vec<String>>();
//    let boards = lines[1..lines.len() - 1];
//    return (calling_order, boards.clone());
//}
