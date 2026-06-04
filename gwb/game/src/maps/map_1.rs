/*
By: Mujibullah
Date: 2026-04-23
Description: This is Map1 module with its functions
 */
use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub struct Map1 {
    mplatform1: StillImage, 
    mplatform2: StillImage,
    platform: StillImage,
    background: StillImage,
    spawn_p1: Vec2,
    spawn_p2: Vec2,
    tm: TextureManager,
}

impl Map1 {
    pub async fn new(tm: TextureManager) -> Self {
        Map1 {
            mplatform1: StillImage::new("", 250.0, 60.0, 120.0, 400.0, true, 1.0).await,
            mplatform2: StillImage::new("", 250.0, 60.0, 1000.0, 500.0, true, 1.0).await,
            platform: StillImage::new("", 400.0, 60.0, 500.0, 380.0, true, 1.0).await,
            background: StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            spawn_p2: vec2(1120.0, 400.0),
            spawn_p1: vec2(160.0, 300.0),
            tm: tm
        }
    }

    pub fn init(&mut self) {
        self.mplatform1.set_preload(self.tm.get_preload("assets/maps/map_1/miniplatform.png").unwrap());
        self.mplatform2.set_preload(self.tm.get_preload("assets/maps/map_1/miniplatform.png").unwrap());
        self.platform.set_preload(self.tm.get_preload("assets/maps/map_1/platform.png").unwrap());
        self.background.set_preload(self.tm.get_preload("assets/maps/map_1/background.png").unwrap());
    }

    #[allow(dead_code)]
    pub fn spawn(&mut self) -> Vec<Vec2>{
        return vec![self.spawn_p1, self.spawn_p2];
    }

    #[allow(dead_code)] 
    pub fn tick(&mut self) {
        {}
    }

    #[allow(dead_code)]
    pub fn draw(&mut self) {
        self.background.draw();
        self.mplatform1.draw();
        self.mplatform2.draw();
        self.platform.draw();
    }

    #[allow(dead_code)]
    pub fn pass_floors(&self) -> Vec<&StillImage> {
        vec![&self.mplatform1, &self.mplatform2, &self.platform]
    }
}
