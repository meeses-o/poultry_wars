/*
By: Mujibullah
Date: 2026-04-23
Description: This is Map1 module with its functions
 */
use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub struct Map1 {
    mplatform1: StillImage, 
    mplatform2: StillImage,
    platform: StillImage,
    background: StillImage,

}

impl Map1 {
    pub async fn new() -> Self {
        Map1 {
            mplatform1: StillImage::new("assets/maps/map_1/miniplatform.png", 200.0, 500.0, 150.0, 450.0, true, 1.0 ).await,
            mplatform2: StillImage::new("assets/maps/map_1/miniplatform.png", 200.0, 60.0, 1000.0, 500.0, true, 1.0).await,
            platform: StillImage::new("assets/maps/map_1/platform.png", 300.0, 60.0, 425.0, 380.0, true, 1.0).await,
            background: StillImage::new("assets/maps/map_1/background.png", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await
        }
    }
    #[allow(dead_code)]
    fn spawn(&mut self) {
        
    }
    #[allow(dead_code)]
    pub fn draw(&mut self) {
        self.mplatform1.draw();
        self.mplatform2.draw();
        self.platform.draw();
        self.background.draw();
    }
}
