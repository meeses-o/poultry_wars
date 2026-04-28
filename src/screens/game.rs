/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{draw_grid};
use crate::classes::chicken::Chicken;
use crate::maps::map1::Map1;

pub async fn run() -> String{
    let mut map1 = Map1::new().await;
    
    let p1 = Chicken::new(640.0, 480.0, true).await;
    let p2 = Chicken::new(340.0, 480.0, true).await;

    loop {
        clear_background(BEIGE);
        map1.draw();
        draw_grid(50.0, BLACK);
        p1.draw();
        p2.draw();
        next_frame().await
    }
}
