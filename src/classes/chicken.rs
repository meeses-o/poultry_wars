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
    jump_posib: bool
}
impl Chicken {
    pub async fn new(x:f32, y:f32, player: String) -> Self {
        Chicken {
            view: StillImage::new(&format!("assets/class_images/chicken/{}/idle_r.png", player), 100.0, 100.0, x, y, true, 1.0).await,
            health: 10,
            health_bar: StillImage::new(&format!("assets/health/{}/health10.png", player), 320.0, 150.0, 650.0, 20.0, true, 1.0).await,
            player: player,
            speed: vec2(0.0,0.0),
            accell: vec2(0.0,0.125),
            jump_posib: true
        }
    }

    pub fn draw(&self) {
        self.view.draw();
        self.health_bar.draw();
    }

    pub fn framemove(&mut self) {
        let topspeedx = 6.5;
        let topspeedy = 5.0;

        //accelleration
        self.speed += self.accell;

        //topspeed limiter on the x
        if self.speed.x > topspeedx {
            self.speed.x = topspeedx;
        } else if self.speed.x < -topspeedx {
            self.speed.x = -topspeedx;
        }

        //topspeed limiter on the y
        if self.speed.y > topspeedy {
            self.speed.y = topspeedy;
        }

        //decelleration
        if self.speed.x > 0.5 {
            self.speed.x -= 0.5;
        } else if self.speed.x < -0.5 {
            self.speed.x += 0.5;
        } else {
            self.speed.x = 0.0;
        }

        //positional movement
        let pos = self.view.get_pos() + self.speed;
        self.view.set_position(pos);
    }

    async fn move_left(&mut self) {
        self.speed.x -= 0.6;
        self.view.set_image(&format!("assets/class_images/chicken/{}/idle_l.png", self.player)).await
    }

    async fn move_right(&mut self) {
        self.speed.x += 0.6;
        self.view.set_image(&format!("assets/class_images/chicken/{}/idle_r.png", self.player)).await
    }

    fn jump(&mut self) {
        self.speed.y -= 12.0;
        self.jump_posib = false;
    }

    //for testing; zero gravity
    pub fn zero_g(&mut self) {
        self.accell.y -= 2.0;
    }

    pub fn check_floor_at(&mut self, floordims: Vec<Vec2>) {
        //This function works that floordims[0] is the pos
        //and [1] is the size. for 0, x = x and y=y.
        //for 1, x=width and y=height. don't use the actual
        //collision module rather coordinates. 

        //Player dims
        let play_top = self.view.get_y();
        let play_bottom = self.view.get_y()+self.view.size().y;
        let play_left = self.view.get_x();
        let play_right = self.view.get_x()+self.view.size().x;

        //Plat dims
        let plat_top = floordims[0].y;
        let plat_bottom = floordims[0].y+floordims[1].y;
        let plat_left = floordims[0].x;
        let plat_right = floordims[0].x+floordims[1].x;
        let plat_mid_y = floordims[0].y+(floordims[1].y/2.0);
        let plat_mid_x = floordims[0].x+(floordims[1].x/2.0);

        //top/bottom of the platform
        if play_bottom > plat_top && play_bottom < plat_mid_y && play_left < plat_right && play_right > plat_left {
            self.view.set_y(plat_top-self.view.size().y);
        } else if play_left < plat_right && play_right > plat_left && play_top < plat_bottom && play_top > plat_mid_y {
            self.view.set_y(plat_bottom);
        }
    }

    pub async fn automove(&mut self) {
        if is_key_down(KeyCode::W) && self.jump_posib{
            self.jump()
        }
        if is_key_down(KeyCode::A) {
            self.move_left().await;
        }

        if is_key_down(KeyCode::D) {
            self.move_right().await;
        }

        self.framemove();
    }

    pub async fn hurt(&mut self) {
        self.health -= 1;
        self.health_bar.set_image(&format!("assets/health/{}/health{}.png", self.player, self.health)).await;
    }
}

//1280x960