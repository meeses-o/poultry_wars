/*
By: Mujibullah
Date: 2026-04-23
Description: This is Map1 module with its functions
 */
use crate::modules::stillimage::StillImage;
use ::macroquad::prelude::*;


pub struct Platform {
    view: StillImage,

}

impl Platform {
    pub async fn new(assets_path: String, ) -> Self {
        Platform {
            view: StillImage::new("res/platform.png"),
        }
    }
}