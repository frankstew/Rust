use macroquad::prelude::*;
use macroquad::rand;

const GRID_SIZE: usize = 200;


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

#[macroquad::main("rust-gui")]
async fn main() {
    
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

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

        let cell_width = screen_width() / GRID_SIZE as f32;
        let cell_height = screen_height() / GRID_SIZE as f32;

        draw_cell(0, 0, true);
        draw_cell(1, 2, true);
        draw_cell(5, 9, true);
        draw_cell(7, 2, true);
        draw_cell(3, 3, true);
        draw_cell(9, 9, true);
        draw_cell(5, 5, true);

        next_frame().await
    }
}
