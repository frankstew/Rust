use shared;

fn main() {
    let input = parse_input("./input.txt");
    let risk = calc_risk(&input);
    println!("{:?}", risk);

}

//storing grid as ["153534920587493", "185839432038429", ....], index by
//grid[i].chars().nth(j).unwrap()
fn parse_input(filename: &str) -> Vec<String> {
    shared::get_lines(filename)
}

fn find_minima(grid: &Vec<String>) -> Vec<(usize, usize)> {
    let mut minima = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_minimum(grid, row, col) {
                minima.push((row, col));
            }
        }
    }
    minima
}

fn calc_risk(grid: &Vec<String>) -> i32 {
    let mut risk = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_minimum(grid, row, col) {
               risk += get_height(grid, row, col) + 1; 
            }
        }
    }
    risk
}

fn is_minimum(grid: &Vec<String>, row: usize, col: usize) -> bool {
    let height = get_height(grid, row, col);
    let mut is_min = true;
    if row == 0 {
        // top
        if col == 0 {
            // top right 
            is_min = is_min && height < get_height(grid, row, col + 1);
            is_min = is_min && height < get_height(grid, row + 1, col);
        } else if col == grid[0].len() - 1 {
            // top right
            is_min = is_min && height < get_height(grid, row, col - 1);
            is_min = is_min && height < get_height(grid, row + 1, col);
        } else {
            // top middle
            is_min = is_min && height < get_height(grid, row, col + 1);
            is_min = is_min && height < get_height(grid, row, col - 1);
            is_min = is_min && height < get_height(grid, row + 1, col);
        }
    } else if row == grid.len() - 1 {
        // bottom
        if col == 0 {
            // bottom right 
            is_min = is_min && height < get_height(grid, row, col + 1);
            is_min = is_min && height < get_height(grid, row - 1, col);
        } else if col == grid[0].len() - 1 {
            // bottom right
            is_min = is_min && height < get_height(grid, row, col - 1);
            is_min = is_min && height < get_height(grid, row - 1, col);
        } else {
            // bottom middle
            is_min = is_min && height < get_height(grid, row, col - 1);
            is_min = is_min && height < get_height(grid, row, col + 1);
            is_min = is_min && height < get_height(grid, row - 1, col);
        }
    } else {
        // middle  
        if col == 0 {
            // middle left 
            is_min = is_min && height < get_height(grid, row, col + 1);
            is_min = is_min && height < get_height(grid, row + 1, col);
            is_min = is_min && height < get_height(grid, row - 1, col);
        } else if col == grid[0].len() - 1 {
            // middle right
            is_min = is_min && height < get_height(grid, row, col - 1);
            is_min = is_min && height < get_height(grid, row + 1, col);
            is_min = is_min && height < get_height(grid, row - 1, col);
        } else {
            // true middle
            is_min = is_min && height < get_height(grid, row, col + 1);
            is_min = is_min && height < get_height(grid, row, col - 1);
            is_min = is_min && height < get_height(grid, row - 1, col);
            is_min = is_min && height < get_height(grid, row + 1, col);
        }
    }
    is_min
}

fn get_height(grid: &Vec<String>, row: usize, col: usize) -> i32 {
    let base: u32 = 10;
    grid[row].chars().nth(col).unwrap().to_digit(base).unwrap() as i32
}
