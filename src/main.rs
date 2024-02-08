use pixel_canvas::{Canvas, Color};
use rand::{thread_rng, Rng};

fn main() {
    let canvas = Canvas::new(512, 512)
        .title("tiles");

    let grid: (usize, usize) = (15, 15);
    let mut tgol = generate_game(0.5, grid);

    let mut count: u64 = 0;

    canvas.render(move |_, image| {
        let width = image.width();
        let height = image.height();

        count += 1;
        if count == 10 {
            count = 0;
            tgol = the_game_of_life(&tgol);
        }

        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = draw_pixel((x, y), &tgol, grid, (width, height));
            }
        }
    });
}

fn draw_pixel(pixel: (usize, usize), tgol_state: &Vec<Vec<bool>>, grid_size: (usize, usize), resolution: (usize, usize)) -> Color {
    let x_position = (pixel.1 as f64 / (resolution.1 as f64 / grid_size.1 as f64)).floor() as usize;
    let y_position = (pixel.0 as f64 / (resolution.0 as f64 / grid_size.0 as f64)).floor() as usize;


    if tgol_state[y_position][x_position] {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    } else {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

// Given a probability and dimentions, generates a grid of cells that are either alive or dead. Higher probabilities will yield more alive cells
fn generate_game(p: f64, dimentions: (usize, usize)) -> Vec<Vec<bool>> {
    let mut rng = thread_rng();
    let mut grid = vec![vec![true; dimentions.1]; dimentions.0];

    for y in grid.iter_mut() {
        for x in y.iter_mut() {
                *x = rng.gen_range(0.0..=1.0) < p
        }
    }

    grid
}

// Returns number of alive neighbors
fn get_alive_neighbors(position: (i8, i8), tgol_state: &Vec<Vec<bool>>) -> u8 {
    let mut alive: u8 = 0;

    for y in -1..=1 {
        for x in -1..=1 {

            let grid_x = x + position.0;
            let grid_y = y + position.1;

            if grid_y < 0 || grid_y  >= tgol_state.len() as i8{
                continue;
            }
            
            if grid_x < 0 || grid_x >= tgol_state[grid_y as usize].len() as i8 {
                continue;
            }

            if tgol_state[grid_y as usize][grid_x as usize] == true && (grid_x, grid_y) != position {
                alive += 1;
            }
        }
    }

    alive
}

// changes the state of a cell based off of the number of alive neighbors
fn change_state(is_alive: bool, neighbors: u8) -> bool {

    match (is_alive, neighbors) {
        (true, 2..=3) => true,
        (false, 3) => true,
        _ => false,
    }
}

fn the_game_of_life(tgol_state: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_state = tgol_state.clone();

    for (y, column) in tgol_state.iter().enumerate() {
        for (x, cell) in column.iter().enumerate() {

            new_state[y][x] = change_state(*cell, get_alive_neighbors((x as i8, y as i8), &tgol_state))
        }
    }

    new_state
}
