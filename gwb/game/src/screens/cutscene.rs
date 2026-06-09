/*
Art: Cameron & Mujib
Coding: Oliver, Mujib

Cutscenes for the game story
 */

use macroquad::prelude::*;
use ureq::get;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::ui::label::Label;

pub async fn run(tm: &TextureManager) {
    // let mut img_chicken = 1;
    let mut img_scenes = 1;
    // let mut chicken = StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await;
    // chicken.set_preload(tm.get_preload(&format!("assets/cutscenes/chicken{}.png", img_chicken)).unwrap());
    let mut scenes = StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await;
    scenes.set_preload(tm.get_preload(&format!("assets/cutscenes/scene{}.png", img_scenes)).unwrap());
    
    let mut scene_time = get_time();
    loop{
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Escape) {
            break
        }
        
        if get_time() - scene_time > 2.0{
            scene_time = get_time();
            img_scenes += 1;
            if img_scenes == 9{
                break
                // img_chicken += 1;
                // chicken.set_preload(tm.get_preload(&format!("assets/cutscenes/chicken{}.png", img_chicken)).unwrap());
            }
            scenes.set_preload(tm.get_preload(&format!("assets/cutscenes/scene{}.png", img_scenes)).unwrap());
        }
        
        clear_background(WHITE);
        scenes.draw();
        next_frame().await;
    }
}
