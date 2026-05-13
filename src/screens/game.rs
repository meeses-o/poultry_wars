/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::classes::chicken::Chicken;
use crate::utils::collision::check_collision;
use crate::maps::map_1::Map;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: &TextureManager) -> bool {
    let mut map = Map::new().await;
    let playerspawn = map.spawn();
    let mut p1 = Chicken::new(tm, playerspawn[0].x, playerspawn[0].y, "p1".to_string(), playerspawn[0]).await;
    let mut p2 = Chicken::new(tm, playerspawn[1].x, playerspawn[1].y, "p2".to_string(), playerspawn[1]).await;
    p1.init();
    p2.init();
    loop {
        //winchecking
        if p2.check_full_dead() {
            return true;
        } else if p1.check_full_dead() {
            return false;
        }

        if is_key_pressed(KeyCode::K) {
            p1.allpurpose_hurt(2);
        }

        clear_background(BEIGE);
        draw_grid(50.0, BLACK);
        map.tick();

        //movement and platform collision
        p1.automove();
        p2.automove();
        let floors = map.pass_floors();
        for floor in &floors {
            p1.check_floor_at(floor.generate_floordims());
            p2.check_floor_at(floor.generate_floordims());
        }

        //attacks
        if p1.get_attack_state() {
            if check_collision(p1.get_view(), p2.get_view(), 5) {
                p2.allpurpose_hurt(1);
            }
        }
        if p2.get_attack_state() {
            if check_collision(p1.get_view(), p2.get_view(), 5) {
                p1.allpurpose_hurt(1);
            }
        }


        //draws
        map.draw();
        p1.draw();
        p2.draw();

        next_frame().await;
    }
}
