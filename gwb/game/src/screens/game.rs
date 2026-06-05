/*
By: Mujibulah
Date: 04/24/2026
Description: This is the map 1 screen
 */
use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::classes::chicken::Chicken;
use crate::classes::turkey::Turkey;
use crate::utils::collision::check_collision;
use crate::maps::map_1::Map1;
use crate::maps::map_2::Map2;
use crate::utils::preload_image::TextureManager;
use crate::utils::player_type::PlayerType;
use crate::utils::map_type::MapType;
use crate::utils::scale::use_virtual_resolution;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};

pub async fn run(tm: &TextureManager, game_info: Vec<String>) -> bool {

    let music = load_sound("assets/pwars_theme.ogg").await.unwrap();
    play_sound(
        &music,
        PlaySoundParams {
            volume: 0.5,
            looped: true
        },
    );
    
    let mut map = if game_info[2] == "dirt".to_string() {
        MapType::Map1(Map1::new(tm.clone()).await)
    } else {
        MapType::Map2(Map2::new(tm.clone()).await)
    };

    map.init();
    let playerspawn = map.spawn();

    let mut p1 = if game_info[0] == "turkey" {
        PlayerType::Turkey(Turkey::new(tm, playerspawn[0].x, playerspawn[0].y, "p1".to_string(), playerspawn[0]).await)
    } else {
        PlayerType::Chicken(Chicken::new(tm, playerspawn[0].x, playerspawn[0].y, "p1".to_string(), playerspawn[0]).await)
    };

    let mut p2 = if game_info[1] == "turkey" {
        PlayerType::Turkey(Turkey::new(tm, playerspawn[1].x, playerspawn[1].y, "p2".to_string(), playerspawn[1]).await)
    } else {
        PlayerType::Chicken(Chicken::new(tm, playerspawn[1].x, playerspawn[1].y, "p2".to_string(), playerspawn[1]).await)
    };

    p1.init();
    p2.init();
    loop {
        use_virtual_resolution(1280.0, 960.0);
        //winchecking
        if p2.check_full_dead() {
            return true;
        } else if p1.check_full_dead() {
            return false;
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
                p2.allpurpose_hurt(p1.get_attack_dmg());
                p2.hit_animate(p1.get_side());
            }
        }
        if p2.get_attack_state() {
            if check_collision(p1.get_view(), p2.get_view(), 5) {
                p1.allpurpose_hurt(p2.get_attack_dmg());
                p1.hit_animate(p2.get_side());
            }
        }

        //draws
        map.draw();
        p1.draw();
        p2.draw();
        
        //artificial lag for frame-independant movement testing
        //std::thread::sleep(std::time::Duration::from_millis(33));
        next_frame().await;
    }
}
