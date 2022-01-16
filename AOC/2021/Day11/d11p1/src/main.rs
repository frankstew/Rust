use shared;
use shared::Grid as Grid;

fn main() {
    let g = parse_input("./input2.txt");
    for row in g.points {
        println!("{:?}", row);
    }
}


fn parse_input(filename: &str) -> Grid {
    let mut g = Grid::default();
    let lines = shared::get_lines(filename);
    let base: u32 = 10;
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(base).unwrap() as i32);
        }
        g.points.push(row);
    }
    g
}

// returns number of flashes in step
fn step(g: &Grid) -> i32 {
    // already flashed?
    // increase energy of octos
    // check for flash and expand out for each octo, checking for flash each time
}
