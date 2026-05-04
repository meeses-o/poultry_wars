/*
By: Mujibullah
Date: 2026-04-23
Description: This is Map1 module with its functions
 */
use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::classes::chicken::Chicken;

pub struct Map {
    mplatform1: StillImage, 
    mplatform2: StillImage,
    platform: StillImage,
    background: StillImage,
    spawn_p1: Vec2,
    spawn_p2: Vec2,
}

impl Map {
    pub async fn new() -> Self {
        Map {
            mplatform1: StillImage::new("assets/maps/map_1/miniplatform.png", 250.0, 100.0, 120.0, 400.0, true, 1.0 ).await,
            mplatform2: StillImage::new("assets/maps/map_1/miniplatform.png", 250.0, 100.0, 1000.0, 500.0, true, 1.0).await,
            platform: StillImage::new("assets/maps/map_1/platform.png", 450.0, 285.0, 450.0, 380.0, true, 1.0).await,
            background: StillImage::new("assets/maps/map_1/background.png", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            spawn_p1: vec2(140.0, 400.0),
            spawn_p2: vec2(1120.0, 500.0),
        }
    }
    #[allow(dead_code)]
    pub fn spawn(&mut self) -> Vec<Vec2>{
        return vec![self.spawn_p1, self.spawn_p2];
    }
    #[allow(dead_code)] 
    fn tick(&mut self) {
        
    }
    #[allow(dead_code)]
    pub fn draw(&mut self) {
        self.background.draw();
        self.mplatform1.draw();
        self.mplatform2.draw();
        self.platform.draw();
    }
}
