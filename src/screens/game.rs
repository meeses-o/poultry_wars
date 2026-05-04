/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::classes::chicken::Chicken;
use crate::maps::map::Map;

pub async fn run() -> String {
    let mut map = Map::new().await;
    let playerspawn = map.spawn();
    let mut p1 = Chicken::new(playerspawn[0].x, playerspawn[0].y, "p1".to_string()).await;

    loop {

        map.draw();
        clear_background(BEIGE);
        draw_grid(50.0, BLACK);

        p1.automove().await;
        p1.floor_at(playerspawn[0].y);
        p1.draw();

        next_frame().await;
    }
}
