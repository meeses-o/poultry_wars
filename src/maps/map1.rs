/*
By: Mujibullah
Date: 2026-04-23
Description: This is Map1 module with its functions
 */
use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub struct Map {
    platform1: StillImage, 
    platform2: StillImage,
    platform3: StillImage,
    background: StillImage,

}

impl Map {
    pub async fn new() -> Self {
        Map {
            platform1: StillImage::new("assets/maps/map_1/miniplatform.png", 200.0, 50.0, 0.0, 0.0, true, 1.0 ).await,
            platform2: StillImage::new("assets/maps/map_1/miniplatform.png", 200.0, 50.0, 0.0, 0.0, true, 1.0).await,
            platform3: StillImage::new("assets/maps/map_1/platform.png", 400.0, 50.0, 0.0, 0.0, true, 1.0).await,
            background: StillImage::new("assets/maps/map_1/background.png", 1024.0, 768.0, 0.0, 0.0, true, 1.0).await
        }
    }
    #[allow(dead_code)]
    fn spawn(&mut self) {
        
    }
    #[allow(dead_code)]
    fn draw(&mut self) {
        self.platform1.draw();
        self.platform2.draw();
        self.platform3.draw();
        self.background.draw();
    }
}
