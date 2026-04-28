//Created by Oliver
//Apr 24th 2026

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub struct Chicken{
    view: StillImage,
    health: i32,
    health_bar: StillImage,
    player: bool,               //p1 = true, p2 = false
    speed: Vec2,                //current speeds in the x and y
    accell: Vec2,               //Current accellerations in the x and y
}
impl Chicken {
    pub async fn new(x:f32, y:f32, player: bool) -> Self {
        if player {
            Chicken {
                view: StillImage::new("assets/class_images/chicken/p1/idle.png", 100.0, 100.0, x, y, true, 1.0).await,
                health: 10,
                health_bar: StillImage::new("assets/health/p1/health10.png", 320.0, 150.0, 310.0, 20.0, true, 1.0).await,
                player: player,
                speed: vec2(0.0,0.0),
                accell: vec2(0.0,-2.0)
            }
        } else {
            Chicken {
                view: StillImage::new("assets/class_images/chicken/p2/idle.png", 100.0, 100.0, x, y, true, 1.0).await,
                health: 10,
                health_bar: StillImage::new("assets/health/p2/health10.png", 320.0, 150.0, 650.0, 20.0, true, 1.0).await,
                player: player,
                speed: vec2(0.0,0.0),
                accell: vec2(0.0,-2.0)
            }
        }
    }

    pub fn draw(&self) {
        self.view.draw();
        self.health_bar.draw();
    }

    pub fn framemove(&mut self) {
        self.speed += self.accell;
    }
}

//1280x960