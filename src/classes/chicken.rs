//Created by Oliver
//Apr 24th 2026

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub struct Chicken{
    view: StillImage,
    health: i32,
    health_bar: StillImage,
    player: String,
    speed: Vec2,                //current speeds in the x and y
    accell: Vec2,               //Current accellerations in the x and y
}
impl Chicken {
    pub async fn new(x:f32, y:f32, player: String) -> Self {
        Chicken {
            view: StillImage::new(&format!("assets/class_images/chicken/{}/idle_r.png", player), 100.0, 100.0, x, y, true, 1.0).await,
            health: 10,
            health_bar: StillImage::new(&format!("assets/health/{}/health10.png", player), 320.0, 150.0, 650.0, 20.0, true, 1.0).await,
            player: player,
            speed: vec2(0.0,0.0),
            accell: vec2(0.0,-2.0)
        }
    }

    pub fn draw(&self) {
        self.view.draw();
        self.health_bar.draw();
    }

    pub fn framemove(&mut self) {
        self.speed += self.accell;
        let pos = self.view.get_pos() + self.speed;
        self.view.set_pos(pos);
    }

    async fn move_left(&mut self) {
        self.speed.x -= 2.0;
        self.view.set_image(&format!("assets/class_images/chicken/{}/idle_l", self.player)).await
    }

    async fn move_right(&mut self) {
        self.speed.x += 2.0;
        self.view.set_image(&format!("assets/class_images/chicken/{}/idle_r", self.player)).await
    }

    fn jump(&mut self) {
        self.speed.y += 5.0;
    }

    pub async fn automove(&mut self) {
        if is_key_pressed(KeyCode::W) {
            self.jump()
        }
        if is_key_pressed(KeyCode::A) {
            self.move_left().await;
        } else {
            self.speed.x += 2.0
        }

        if is_key_pressed(KeyCode::S) {
            self.move_right().await;
        } else {
            self.speed.x -= 2.0
        }

        self.framemove();
    }
}

//1280x960