//created by cameron

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub struct Map2 {
    barplatform: StillImage,
    mapfloor: StillImage,
    background: StillImage,
    cranechain: StillImage,
    spawn_p1: Vec2,
    spawn_p2: Vec2,
    tm: TextureManager,
}

impl Map2 {
    pub async fn new(tm: TextureManager) -> Self {
        Map2 {
            barplatform: StillImage::new("",80.0, 15.0, 500.0, 400.0, true, 1.0).await,
            mapfloor: StillImage::new("",1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            background: StillImage::new("",1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            cranechain: StillImage::new("", 20.0, 200.0, 500.0, 200.0, true, 1.0).await,
            spawn_p1: Vec2::new(1120.0, 400.0),
            spawn_p2: Vec2::new(160.0, 400.0),
            tm:tm,
        }
    }

    pub fn init(&mut self) {
        self.barplatform.set_preload(self.tm.get_preload("assets/maps/map_2/cranebar.png").unwrap());
        self.mapfloor.set_preload(self.tm.get_preload("assets/maps/map_2/mapfloors.png").unwrap());
        self.background.set_preload(self.tm.get_preload("assets/maps/map_2/citymap.png").unwrap());
        self.cranechain.set_preload(self.tm.get_preload("assets/maps/map_2/cranechain.png").unwrap());
    }
    #[allow(dead_code)]
    pub fn tick(&self) {
        {}
    }
    #[allow(dead_code)]
    pub fn draw(&self) {
        self.cranechain.draw();
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