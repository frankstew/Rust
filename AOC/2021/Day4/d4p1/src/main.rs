use shared;
use std::collections::HashSet;




fn main() {
    let (calling_order, mut boards) = parse_bingo_input("./input.txt");
    println!("{}", find_last_winning_board(calling_order, &mut boards));
    //println!("{}", calling_order[0]);
    //println!("{}", boards[0][0][0].0);
    //let mut board = Vec::new();
    //board.push(vec![(1, false), (2, true), (3, true), (4, true), (5, true)]);
    //board.push(vec![(6, false), (7, true), (8, true), (9, false), (10, false)]);
    //board.push(vec![(11, false), (12, true), (13, true), (14, false), (15, false)]);
    //board.push(vec![(16, false), (17, false), (18, true), (19, false), (20, false)]);
    //board.push(vec![(21, false), (22, true), (23, true), (24, false), (25, false)]);
    //print_board(board.clone());
    //number_called(16, &mut board);
    //println!("{}", is_bingo(board));
    //print_board(board);
}

fn parse_bingo_input(filename: &str) -> (Vec<i32>, Vec<Vec<Vec<(i32, bool)>>>) {
    let lines = shared::get_lines(filename);
    let calling_order = lines[0].to_owned().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let board_lines = &lines[1..lines.len()];
    let owned_board_lines = board_lines.to_owned();
    let mut split_board_lines = Vec::new();
    for board_line in owned_board_lines.iter() {
        let i: Vec<(i32, bool)> = board_line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).map(|num| (num, false)).collect();
        split_board_lines.push(i);
    }
    let boards: Vec<Vec<Vec<(i32, bool)>>> = split_board_lines.chunks(5).map(|n| n.to_owned()).collect();
    return (calling_order, boards);
}

fn is_bingo(board: Vec<Vec<(i32, bool)>>) -> bool {
    // columns
    for row_index in 0..board.len() {
        let mut row_sum = 0;
        for column_index in 0..board.len() {
            if (board[row_index][column_index].1 == true) {
                row_sum += 1;
            }
            // could break here if false bc need entire row/col for bingo
            //println!("({}, {}): {}", board[row_index][column_index].0, board[row_index][column_index].1, row_sum);
        }
        if (row_sum == 5) {
            return true;
        }
        row_sum = 0;
    }
    // rows
    for column_index in 0..board.len() {
        let mut col_sum = 0;
        for row_index in 0..board.len() {
            if (board[row_index][column_index].1 == true) {
                col_sum += 1;
            }
            // could break here if false bc need entire row/col for bingo
            //println!("({}, {}): {}", board[row_index][column_index].0, board[row_index][column_index].1, col_sum);
        }
        if (col_sum == 5) {
            return true;
        }
        col_sum = 0;
    }
    return false;
}

fn number_called(num: i32, board: &mut Vec<Vec<(i32, bool)>>) {
    for column_index in 0..board.len() {
        for row_index in 0..board.len() {
            let entry = board[row_index][column_index]; 
            if (entry.0 == num) {
                print_board(board.clone());
                println!();
                board[row_index][column_index].1 = true;
                return;
            }
        }
    }
    return;
}

fn sum_unselected_squares(board: Vec<Vec<(i32, bool)>>)  -> i32 {
    let mut square_sum = 0;
    for column_index in 0..board.len() {
        for row_index in 0..board.len() {
            let entry = board[column_index][row_index];
            if (!entry.1) {
                square_sum += entry.0;
            }
        }
    }
    square_sum
}

fn find_first_winning_board(calling_order: Vec<i32>, boards: &mut Vec<Vec<Vec<(i32, bool)>>>) -> i32 {
    for num_index in 0..calling_order.len() {
        for mut board in &mut *boards {
            number_called(calling_order[num_index], &mut board);
            //println!("{}", num);
            println!("{}", calling_order[num_index]);
            if (is_bingo(board.clone())) {
                return sum_unselected_squares(board.clone()) * calling_order[num_index];
            }
        }
    }
    return -1;
}

fn find_last_winning_board(calling_order: Vec<i32>, boards: &mut Vec<Vec<Vec<(i32, bool)>>>) -> i32 {
    // this is a really ratchety way of doing this, makes me feel dirty but gets the job done and I'm not gonna fix it for a while
    let mut bingo_indices = HashSet::new();
    let length = boards.len();
    for num_index in 0..calling_order.len() {
        for board_index in 0..length {
            let mut board = &mut (&mut *boards)[board_index];
            number_called(calling_order[num_index], board);
            //println!("{}", num);
            println!("num called: {}", calling_order[num_index]);
            if (is_bingo(board.clone())) {
                bingo_indices.insert(board_index);
                if (bingo_indices.len() == length) {
                    println!("{} * {}", sum_unselected_squares(board.clone()), calling_order[num_index]);
                    return sum_unselected_squares(board.clone()) * calling_order[num_index];
                }
            }
        }
    }
    return -1;
}

fn print_board(board: Vec<Vec<(i32, bool)>>) {
    println!("{}{} {}{} {}{} {}{} {}{}", star_or_empty(&board[0][0]), board[0][0].0, star_or_empty(&board[0][1]), board[0][1].0, star_or_empty(&board[0][2]), board[0][2].0, star_or_empty(&board[0][3]), board[0][3].0, star_or_empty(&board[0][4]), board[0][4].0);
    println!("{}{} {}{} {}{} {}{} {}{}", star_or_empty(&board[1][0]), board[1][0].0, star_or_empty(&board[1][1]), board[1][1].0, star_or_empty(&board[1][2]), board[1][2].0, star_or_empty(&board[1][3]), board[1][3].0, star_or_empty(&board[1][4]), board[1][4].0);
    println!("{}{} {}{} {}{} {}{} {}{}", star_or_empty(&board[2][0]), board[2][0].0, star_or_empty(&board[2][1]), board[2][1].0, star_or_empty(&board[2][2]), board[2][2].0, star_or_empty(&board[2][3]), board[2][3].0, star_or_empty(&board[2][4]), board[2][4].0);
    println!("{}{} {}{} {}{} {}{} {}{}", star_or_empty(&board[3][0]), board[3][0].0, star_or_empty(&board[3][1]), board[3][1].0, star_or_empty(&board[3][2]), board[3][2].0, star_or_empty(&board[3][3]), board[3][3].0, star_or_empty(&board[3][4]), board[3][4].0);
    println!("{}{} {}{} {}{} {}{} {}{}", star_or_empty(&board[4][0]), board[4][0].0, star_or_empty(&board[4][1]), board[4][1].0, star_or_empty(&board[4][2]), board[4][2].0, star_or_empty(&board[4][3]), board[4][3].0, star_or_empty(&board[4][4]), board[4][4].0);
}


fn star_or_empty(board_entry: &(i32, bool)) -> String {
    if (board_entry.1 == true) {
        return String::from("*");
    }
    return String::from("");
}
