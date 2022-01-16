use shared;
use shared::Grid as Grid;
use shared::Point as Point;
use std::collections::HashSet;

fn main() {
    let mut g = parse_input("./input.txt");
    print_grid(&g);

    let steps = 100;
    let f = get_flashes_after_steps(steps, &mut g);
    println!("{} flashes after {} steps", f, steps);

    //println!("First synchro flash is at step {}", find_first_synchro_flash(&mut g));
     
}

fn print_grid(g: &Grid) {
    for row in &g.points {
        print!("[");
        for p in row {
            if p == &0 {
                print!("*");
            } else {
                print!(" ");
            }
            print!("{} ", p);
        }
        println!("]");
    }
}

fn find_first_synchro_flash(g: &mut Grid) -> usize {
    let mut num_octos = 0;
    for row in &g.points {
        num_octos += row.iter().count();
    }
    let mut step_num = 1;
    loop {
        let flashes = step(g);
        if flashes == num_octos || step_num == 10000 {
            return step_num; 
        }
        step_num += 1;
    }
}

fn get_flashes_after_steps(n: i32, g: &mut Grid) -> usize {
    let every_ten: bool = n > 10;
    let mut flashes = 0;
    for i in 0..n {
        flashes += step(g);
        if every_ten {
            if (i + 1) % 10 == 0 {
                println!("");
                println!("after step {}", i + 1);
                print_grid(g);
            }

        } else {
            println!("");
            println!("after step {}", i + 1);
            print_grid(g);
        }
    }
    flashes
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

fn increment_octo_energy(g: &mut Grid) {
    let rows = &g.points.len();
    let cols = &g.points[0].len();
    for row in 0..*rows {
        for col in 0..*cols {
            g.points[row][col] += 1;
        }
    }
}

fn flash_octos(g: &mut Grid, flashed: &mut HashSet<Point>) {
    let rows = &g.points.len();
    let cols = &g.points[0].len();
    for row in 0..*rows {
        for col in 0..*cols {
            let p = Point { x: row, y: col };
            if g.get_point_val(p.x, p.y) > 9 && !flashed.contains(&p) {
                flashed.insert(p.clone());
                flash(&p, g, flashed);
            }
        }
    }
}

fn set_flashed_octo_energy(g: &mut Grid, flashed: &HashSet<Point>) {
    for p in flashed {
        g.points[p.x][p.y] = 0;
    }
}

fn flash(point: &Point, g: &mut Grid, flashed: &mut HashSet<Point>) {
    let adj = g.get_adjacent_points_inc_diag(point.x, point.y);
    for p in adj {
        g.points[p.x][p.y] += 1;
        if g.get_point_val(p.x, p.y) > 9 && !flashed.contains(&p) {
            flashed.insert(p.clone());
            let  new_point = Point { x: p.x, y: p.y };
            flash(&new_point, g, flashed);
        }
    }
}

// returns number of flashes in step
fn step(g: &mut Grid) -> usize {
    let mut flashed: HashSet<Point> = HashSet::new();
    
    increment_octo_energy(g);
    flash_octos(g, &mut flashed);
    set_flashed_octo_energy(g, &flashed);

    flashed.len()
}

