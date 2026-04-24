/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{draw_grid};

pub async fn run() -> String{

    loop {
        clear_background(BEIGE);
        draw_grid(50.0, BLACK);
        next_frame().await
    }
}