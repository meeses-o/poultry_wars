/*
Art: Cameron & Mujib
Coding: Mujib

Cutscenes for the game story
 */

//use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
//use macroquad::experimental::collections::storage::get;
use macroquad::prelude::*;

pub async fn run(tm: &TextureManager) {
    let mut img_chicken = 0;
    let mut img_scenes = 1;
    let mut chicken = StillImage::new("", 64.0, 64.0, 1280.0, 515.0, true, 1.0).await;
    //   chicken.set_preload(tm.get_preload(&format!("")).unwrap());
    let mut scenes = StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await;
    scenes.set_preload(tm.get_preload(&format!("assets/cutscenes/scene{}.png", img_scenes)).unwrap());

    let mut scene_time = get_time();

    chicken.set_preload(tm.get_preload("assets/cutscenes/evil_chicken.png").unwrap());
    chicken.set_angle(45.0);

    loop {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if img_scenes <= 8 {
            chicken.set_x(chicken.get_x() - 70.0*get_frame_time() as f32);
        }
        if get_time() - scene_time > 2.0 {
            //&& img_scenes <= 8{
            scene_time = get_time();
            img_scenes += 1;

            if img_scenes <= 8 {
                scenes.set_preload(tm.get_preload(&format!("assets/cutscenes/scene{}.png", img_scenes)).unwrap());
            }
            if img_scenes == 4 {
                chicken.set_preload(tm.get_preload("assets/cutscenes/the_murderer.png").unwrap());
            }

            if img_scenes > 8 {
                img_chicken += 1;
                if img_chicken <= 3 {
                    scenes.set_preload(tm.get_preload(&format!("assets/cutscenes/chicken{}.png", img_chicken)).unwrap());
                }
            }
            if img_chicken > 3 && img_scenes > 8 {
                break;
            }
        }

        clear_background(WHITE);
        scenes.draw();
        if img_scenes <= 8 {
            chicken.draw();
        }
        next_frame().await;
    }
}
