/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{draw_grid};
use crate::classes::chicken::Chicken;
use crate::maps::map::Map;

pub async fn run() -> String{
    let mut map = Map::new().await;
    let playerspawn = map.spawn();
    let mut p1 = Chicken::new(playerspawn[0].x, playerspawn[0].y, "p1".to_string()).await;
    let mut p2 = Chicken::new(playerspawn[1].x, playerspawn[1].y, "p2".to_string()).await;

    loop {
        map.draw();
        draw_grid(50.0, BLACK);

        p1.automove().await;
        p2.automove().await;
        p1.draw();
        p2.draw();

        next_frame().await
    }
}
