//created by cameron
//date: 2026-05-12
//map 2 module with background, floors, and a moving balloon that loops every 10 seconds

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub struct Map2 {
    barplatform: StillImage,
    leftfloor: StillImage,
    rightfloor: StillImage,
    background: StillImage,
    cranechain: StillImage,
    balloon: StillImage, // Added missing field
    spawn_p1: Vec2,
    spawn_p2: Vec2,
    tm: TextureManager,
    balloon_timer: f32, // Added to track the 10-second loop
    last_bkg_time:f64,
    bkg_img_counter:usize,
}

impl Map2 {
    pub async fn new(tm: TextureManager) -> Self {
        Map2 {
            barplatform: StillImage::new("", 150.0, 15.0, 579.5, 580.0, true, 1.0).await,
            leftfloor: StillImage::new("", 459.0, 290.0, 1.0, 690.0, true, 1.0).await,
            rightfloor: StillImage::new("", 388.0, 290.0, 900.0, 690.0, true, 1.0).await,
            background: StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await,
            cranechain: StillImage::new("", 15.0, 198.0, 650.0, 385.0, true, 1.0).await,
            balloon: StillImage::new("", 100.0, 120.0, -100.0, 592.7, true, 1.0).await,
            spawn_p1: Vec2::new(1120.0, 400.0),
            spawn_p2: Vec2::new(160.0, 400.0),
            tm: tm,
            balloon_timer: 0.0,
            last_bkg_time: 0.0,
            bkg_img_counter: 1,
        }
    }

    pub fn init(&mut self) {
        self.barplatform.set_preload(self.tm.get_preload("assets/maps/map_2/cranebar.png").unwrap());
        self.leftfloor.set_preload(self.tm.get_preload("assets/maps/map_2/left_floor.png").unwrap());
        self.rightfloor.set_preload(self.tm.get_preload("assets/maps/map_2/right_floor.png").unwrap());
        self.background.set_preload(self.tm.get_preload("assets/maps/map_2/citymap.png").unwrap());
        self.cranechain.set_preload(self.tm.get_preload("assets/maps/map_2/cranechain.png").unwrap());
        self.balloon.set_preload(self.tm.get_preload("assets/maps/map_2/airballoon.png").unwrap());
    }

    pub fn tick(&mut self) {
        let dt = get_frame_time();
        self.balloon_timer += dt;

        // Reset loop every 25 seconds
        if self.balloon_timer >= 25.0 {
            self.balloon_timer = 0.0;
            self.balloon.set_x(-100.0);
            self.balloon.set_y(592.7);
        }

        // Only move the balloon if it hasn't left the right/top edges of the screen yet
        if self.balloon.get_x() < 1280.0 && self.balloon.get_y() > -120.0 {
            // Speeds calibrated to cross the map cleanly in ~5.5 seconds
            self.balloon.set_x(self.balloon.get_x()+(250.9 * dt * 0.25));
            self.balloon.set_y(self.balloon.get_y()+(-153.8 * dt * 0.25));
        }

        // if get_time() - self.last_bkg_time > 3.0 {
        //     if self.bkg_img_counter > 6 {   //TODO: Set limit to total frames made
        //         self.bkg_img_counter = 1;
        //     }
        //     self.background.set_preload(self.tm.get_preload(&format!("bkg_{}.png", self.bkg_img_counter)).unwrap());
        // }
    }

    pub fn draw(&self) {
        self.background.draw();
        
        // "Erased" from visible area: Only draw if it's within the screen borders
        if self.balloon.get_x() >= -100.0 && self.balloon.get_x() < 1280.0 && self.balloon.get_y() > -120.0 && self.balloon.get_y() < 960.0 {
            self.balloon.draw();
        }

        self.cranechain.draw();
        self.leftfloor.draw();
        self.rightfloor.draw();
        self.barplatform.draw();
    }

    #[allow(dead_code)]
    pub fn pass_floors(&self) -> Vec<&StillImage> {
        vec![&self.leftfloor, &self.rightfloor, &self.barplatform]
    }

    #[allow(dead_code)]
    pub fn spawn(&self) -> Vec<Vec2> {
        return vec![self.spawn_p1, self.spawn_p2];
    }
}