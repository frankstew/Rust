use macroquad::prelude::*;
use std::{thread, time};

const GRID_SIZE: usize = 100;


fn draw_cell(row: usize, col: usize, is_alive: bool) {
    // +1 and -2 so that I don't draw over the grid lines
    let x = (screen_width() / GRID_SIZE as f32) * col as f32 + 1.0;
    let y = (screen_height() / GRID_SIZE as f32) * row as f32 + 1.0;

    let cell_width = screen_width() / GRID_SIZE as f32 - 2.0;
    let cell_height = screen_height() / GRID_SIZE as f32 - 2.0;

    if is_alive {
        draw_rectangle(x, y, cell_width, cell_height, BLACK);
        return;
    }

    draw_rectangle(x, y, cell_width, cell_height, WHITE);
}

fn get_cell_from_mouse_pos(xpos: f32, ypos: f32) -> (usize, usize) {
    //i * cellwidth + something = xpos
    //i = (xpos - something) / cellwodth = xpos / cellwidth - something / cellwidth
    let cell_width = screen_width() / GRID_SIZE as f32;
    let cell_height = screen_height() / GRID_SIZE as f32;

    let row = (ypos / cell_height) as usize;
    let col = (xpos / cell_width) as usize;
    return (row, col);
}

fn toggle_cell_alive(mouse_pos: Vec2, grid: &mut Vec<Vec<bool>>) {
    let mouse_x = mouse_pos[0];
    let mouse_y = mouse_pos[1];

    let (row, col) = get_cell_from_mouse_pos(mouse_x, mouse_y);
    grid[row][col] = !grid[row][col];

}

struct Neighbours {
    alive: i8,
}

impl Neighbours {
    fn new() -> Self {
        let n = Neighbours {
            alive: 0,
        };
        return n;
    }
}

fn get_neighbours(row: usize, col: usize, grid: &mut Vec<Vec<bool>>) -> Neighbours {
    let mut n = Neighbours::new();

    let above = (row + GRID_SIZE - 1) % GRID_SIZE;
    let below = (row + 1) % GRID_SIZE;
    let left = (col + GRID_SIZE - 1) % GRID_SIZE;
    let right = (col + 1) % GRID_SIZE;


    let top_row = grid.get(above);
    match top_row {
        Some(row) => {
            match row.get(left) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
            match row.get(col) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
            match row.get(right) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
        },
        None => {},
        Some(_) => {}
    }

    let middle_row = grid.get(row);
    match middle_row {
        Some(row) => {
            match row.get(left) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
            match row.get(right) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
        },
        None => {},
        Some(_) => {}
    }

    let bottom_row = grid.get(below);
    match bottom_row {
        Some(row) => {
            match row.get(left) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
            match row.get(col) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
            match row.get(right) {
                Some(val) if *val => n.alive+=1,
                None => {},
                Some(_) => {}
            };
        },
        None => {},
        Some(_) => {}
    }

    return n;
}

fn apply_rules(grid: &mut Vec<Vec<bool>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let neighbours = get_neighbours(i, j, grid);
            if (grid[i][j] && (neighbours.alive == 2 || neighbours.alive == 3)) {
                continue;
            }
            if (!grid[i][j] && neighbours.alive == 3) {
                grid[i][j] = true;
                continue;
            }
            grid[i][j] = false;
        }
    }

}


#[macroquad::main("rust-gui")]
async fn main() {
        let mut grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
        let mut run: bool = false;

    loop {
        clear_background(WHITE);

        for i in 0..GRID_SIZE {
            let y = screen_height() / GRID_SIZE as f32 * i as f32;
            draw_line(0.0, y, screen_width(), y, 1.0, BLACK);
        }
        

        for j in 0..GRID_SIZE {
            let x = screen_width() / GRID_SIZE as f32 * j as f32;
            draw_line(x, 0.0, x, screen_height(), 1.0, BLACK);
        }

        if (run) {
            apply_rules(&mut grid);


            thread::sleep(time::Duration::from_secs(1));
        } 

        if is_mouse_button_pressed(MouseButton::Left) { toggle_cell_alive(mouse_position().into(), &mut grid); }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                draw_cell(i, j, grid[i][j]);
            }
        }

        if is_key_pressed(KeyCode::Enter) {
            run = !run;
        }

        
        next_frame().await
    }
}
