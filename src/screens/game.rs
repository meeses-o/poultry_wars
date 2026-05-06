/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::ui::still_image::StillImage;
use crate::classes::chicken::Chicken;
use crate::maps::map_1::Map;


pub async fn run() -> String {
    let mut map = Map::new().await;
    let playerspawn = map.spawn();
    let mut p1 = Chicken::new(playerspawn[0].x, playerspawn[0].y, "p1".to_string(), playerspawn[0]).await;
    loop {
        clear_background(BEIGE);
        draw_grid(50.0, BLACK);
        map.tick();

        p1.automove().await;
        let floors = map.pass_floors();
        for floor in &floors {
            p1.check_floor_at(floor.generate_floordims()).await;
        }

        map.draw();
        p1.draw();
        next_frame().await;
    }
}
