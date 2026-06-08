/*
Art: Cameron & Mujib
Coding: Oliver, Mujib

Cutscenes for the game story
 */

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::ui::label::Label;

pub async fn run(tm: TextureManager)->String {
    let mut scenestart = StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await;
    scenestart.set_preload(tm.get_preload("assets/cutscenes/scenestart.png").unwrap());
    

    loop{
        
        clear_background(WHITE);
        scenestart.draw();
        next_frame().await;
    }
}
