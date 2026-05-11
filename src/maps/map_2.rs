//created by cameron

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub struct Map {
    barplatform: StillImage,
    mapfloor: StillImage,
    background: StillImage,
    spawn_p1: Vec2,
    spawn_p2: Vec2,
}

impl Map {
    pub async fn new() -> Self {
        Map {
            barplatform: StillImage::new("assets/maps/map_2/cranebar.png", 80.0, 15.0, 500.0, 400.0, true, 1.0).await,
            mapfloor: StillImage::new("assets/mapfloor.png", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            background: StillImage::new("assets/map2background.png", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            spawn_p1: Vec2::new(100.0, 600.0),
            spawn_p2: Vec2::new(700.0, 600.0),
        }
    }
    #[allow(dead_code)]
    pub fn tick(&self) {
        {}
    }
    #[allow(dead_code)]
    pub fn draw(&self) {
        self.background.draw();
        self.mapfloor.draw();
        self.barplatform.draw();
    }
    #[allow(dead_code)]
    pub fn pass_floors(&self) -> Vec<&StillImage> {
        vec![&self.mapfloor, &self.barplatform]
    }
    #[allow(dead_code)]
    pub fn spawn(&self) -> Vec<Vec2> {
        return vec![self.spawn_p1, self.spawn_p2];
    }
 }