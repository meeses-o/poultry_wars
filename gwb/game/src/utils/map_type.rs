//Created by oliver
//2026-05-24
//Used to connect MapType to the map modules.

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::maps::map_1::Map1;
use crate::maps::map_2::Map2;

pub enum MapType {
    Map1(Map1),
    Map2(Map2),
}

impl MapType {
    pub fn init(&mut self) {
        match self {
            MapType::Map1(m) => m.init(),
            MapType::Map2(m) => m.init(),
        }
    }

    pub fn spawn(&mut self) -> Vec<Vec2> {
        match self {
            MapType::Map1(m) => m.spawn(),
            MapType::Map2(m) => m.spawn(),
        }
    }

    pub fn tick(&mut self) {
        match self {
            MapType::Map1(m) => m.tick(),
            MapType::Map2(m) => m.tick(),
        }
    }

    pub fn draw(&mut self) {
        match self {
            MapType::Map1(m) => m.draw(),
            MapType::Map2(m) => m.draw(),
        }
    }

    pub fn pass_floors(&self) -> Vec<&StillImage> {
        match self {
            MapType::Map1(m) => m.pass_floors(),
            MapType::Map2(m) => m.pass_floors(),
        }
    }
}