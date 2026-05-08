/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::classes::chicken::Chicken;
use crate::maps::map_1::Map;


pub async fn run() -> String {
    let mut map = Map::new().await;
    let playerspawn = map.spawn();
    let mut p1 = Chicken::new(playerspawn[0].x, playerspawn[0].y, "p1".to_string(), playerspawn[0]).await;
    let mut p2 = Chicken::new(playerspawn[1].x, playerspawn[1].y, "p2".to_string(), playerspawn[1]).await;
    loop {
        clear_background(BEIGE);
        draw_grid(50.0, BLACK);
        map.tick();

        //movement and platform collision
        p1.automove().await;
        p2.automove().await;
        let floors = map.pass_floors();
        for floor in &floors {
            p1.check_floor_at(floor.generate_floordims()).await;
            p2.check_floor_at(floor.generate_floordims()).await;
        }

        //attacks
        


        //draws
        map.draw();
        p1.draw();
        p2.draw();

        //winchecking
        if p2.check_full_dead() {
            return "p1_win".to_string();
        } else if p1.check_full_dead() {
            return "p2_win".to_string();
        }

        next_frame().await;
    }
}
