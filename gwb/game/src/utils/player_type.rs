//Created by oliver
//2026-05-24
//Used to connect PlayerType to the player modules.

use macroquad::prelude::*;
use crate::classes::chicken::Chicken;
use crate::classes::turkey::Turkey;
use crate::ui::still_image::StillImage;

pub enum PlayerType {
    Chicken(Chicken),
    Turkey(Turkey),
}

impl PlayerType {
    pub fn init(&mut self) {
        match self {
            PlayerType::Chicken(c) => c.init(),
            PlayerType::Turkey(t) => t.init(),
        }
    }

    pub fn draw(&self) {
        match self {
            PlayerType::Chicken(c) => c.draw(),
            PlayerType::Turkey(t) => t.draw(),
        }
    }

    pub fn hit_animate(&mut self, op_side: bool) {
        match self {
            PlayerType::Chicken(c) => c.hit_animate(op_side),
            PlayerType::Turkey(t) => t.hit_animate(op_side),
        }
    }

    pub fn get_side(&mut self) -> bool{
        match self {
            PlayerType::Chicken(c) => c.get_side(),
            PlayerType::Turkey(t) => t.get_side(),
        }
    }

    pub fn automove(&mut self) {
        match self {
            PlayerType::Chicken(c) => c.automove(),
            PlayerType::Turkey(t) => t.automove(),
        }
    }

    pub fn allpurpose_hurt(&mut self, dmg: i32) {
        match self {
            PlayerType::Chicken(c) => c.allpurpose_hurt(dmg),
            PlayerType::Turkey(t) => t.allpurpose_hurt(dmg),
        }
    }

    pub fn check_floor_at(&mut self, dims: Vec<Vec2>) {
        match self {
            PlayerType::Chicken(c) => c.check_floor_at(dims),
            PlayerType::Turkey(t) => t.check_floor_at(dims),
        }
    }

    pub fn check_full_dead(&self) -> bool {
        match self {
            PlayerType::Chicken(c) => c.check_full_dead(),
            PlayerType::Turkey(t) => t.check_full_dead(),
        }
    }

    pub fn get_view(&self) -> &StillImage {
        match self {
            PlayerType::Chicken(c) => c.get_view(),
            PlayerType::Turkey(t) => t.get_view(),
        }
    }

    pub fn get_attack_state(&self) -> bool {
        match self {
            PlayerType::Chicken(c) => c.get_attack_state(),
            PlayerType::Turkey(t) => t.get_attack_state(),
        }
    }

    pub fn get_attack_dmg(&self) -> i32 {
        match self {
            PlayerType::Chicken(c) => c.get_attack_dmg(),
            PlayerType::Turkey(t) => t.get_attack_dmg(),
        }
    }
}