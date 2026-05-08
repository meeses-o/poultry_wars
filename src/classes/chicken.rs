//Created by Oliver
//Apr 24th 2026

use macroquad::{experimental::collections::storage::get, prelude::*};
use crate::ui::still_image::StillImage;

pub struct Chicken{
    view: StillImage,
    health: i32,
    stocks: usize,
    health_bar: StillImage,
    stock_counter: StillImage,
    player: String,
    speed: Vec2,                //current speeds in the x and y
    accell: Vec2,               //Current accellerations in the x and y
    jump_posib: bool,
    start_pos: Vec2,
    attack: bool,
    last_attack_time: f64,
}
impl Chicken {
    pub async fn new(x:f32, y:f32, player: String, start_pos: Vec2) -> Self {
        if player == "p1".to_string() {
            Chicken {
                view: StillImage::new(&format!("assets/class_images/chicken/{}/idle_r.png", player), 50.0, 100.0, x, y, true, 1.0).await,
                health: 10,
                stocks: 3,
                health_bar: StillImage::new(&format!("assets/health/{}/health10.png", player), 320.0, 200.0, 650.0, -38.0, true, 1.0).await,
                stock_counter: StillImage::new("assets/health/stocks/3.png", 100.0, 100.0, 650.0, 50.0, true, 1.0).await,
                player: player,
                speed: vec2(0.0,0.0),
                accell: vec2(0.0,0.125),
                jump_posib: true,
                start_pos: start_pos,
                attack: false,
                last_attack_time: -0.5,
            }  
        } else {
            Chicken {
                view: StillImage::new(&format!("assets/class_images/chicken/{}/idle_r.png", player), 50.0, 100.0, x, y, true, 1.0).await,
                health: 10,
                stocks: 3,
                health_bar: StillImage::new(&format!("assets/health/{}/health10.png", player), 320.0, 200.0, 320.0, -34.0, true, 1.0).await,
                stock_counter: StillImage::new("assets/health/stocks/3.png", 100.0, 100.0, 320.0, 50.0, true, 1.0).await,
                player: player,
                speed: vec2(0.0,0.0),
                accell: vec2(0.0,0.125),
                jump_posib: true,
                start_pos: start_pos,
                attack: false,
                last_attack_time: -0.5,
            }
        }
    }

    pub fn draw(&self) {
        self.health_bar.draw();
        self.stock_counter.draw();
        self.view.draw();
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
        if !self.attack {
            self.view.set_image(&format!("assets/class_images/chicken/{}/idle_l.png", self.player)).await;
        } else {
            self.view.set_image(&format!("assets/class_images/chicken/{}/attack_l.png", self.player)).await;
        }
    }

    async fn move_right(&mut self) {
        self.speed.x += 0.6;
        if !self.attack {
            self.view.set_image(&format!("assets/class_images/chicken/{}/idle_r.png", self.player)).await;
        } else {
            self.view.set_image(&format!("assets/class_images/chicken/{}/attack_r.png", self.player)).await;
        }
    }

    fn jump(&mut self) {
        self.speed.y = -7.0;
        self.jump_posib = false;
    }

    async fn attack(&mut self) {
        let time = get_time();
        if time - self.last_attack_time > 0.5 {
            self.attack = true;
            self.last_attack_time = time;
        }
    }

    //for testing; zero gravity
    pub fn zero_g(&mut self) {
        self.accell.y -= 2.0;
    }

    pub fn reset_pos(&mut self) {
        self.view.set_position(self.start_pos);
    }

    pub async fn allpurpose_hurt(&mut self) {
        if self.health < 1 && self.stocks > 0 {
            self.stocks -= 1;
            self.health = 10;
            self.view.set_position(self.start_pos);
        } else {
            self.health -= 1;
        }
        self.health_bar.set_image(&format!("assets/health/{}/health{}.png", self.player, self.health)).await;
        self.stock_counter.set_image(&format!("assets/health/stocks/{}.png", self.stocks)).await;
    }

    pub fn check_full_dead(&self) -> bool {
        if self.stocks > 0 {
            return false;
        } else {
            return true;
        }
    }

    pub async fn check_floor_at(&mut self, floordims: Vec<Vec2>) {
        //This function works that floordims[0] is the pos
        //and [1] is the size. for 0, x = x and y=y.
        //for 1, x=width and y=height. don't use the actual
        //collision module rather coordinates. 

        //Player dims
        let mut play_top = self.view.get_y();
        let mut play_bottom = self.view.get_y()+self.view.size().y;
        let play_left = self.view.get_x();
        let play_right = self.view.get_x()+self.view.size().x;

        //Plat dims
        let plat_top = floordims[0].y;
        let plat_bottom = floordims[0].y+floordims[1].y;
        let plat_left = floordims[0].x;
        let plat_right = floordims[0].x+floordims[1].x;
        let plat_mid_y = floordims[0].y+(floordims[1].y/2.0);
        let plat_mid_x = floordims[0].x+(floordims[1].x/2.0);

        let mut temp_jump_possib = false;

        //top/bottom of the platform
        if play_bottom > plat_top && play_bottom < plat_mid_y && play_left < plat_right && play_right > plat_left { //top
            self.view.set_y(plat_top-self.view.size().y);
            temp_jump_possib = true;
        } else if play_left < plat_right && play_right > plat_left && play_top < plat_bottom && play_top > plat_mid_y { //bott
            self.view.set_y(plat_bottom);
            temp_jump_possib = false;
        }

        play_top = self.view.get_y();
        play_bottom = self.view.get_y()+self.view.size().y;

        //left/right of the platform
        if play_bottom > plat_top && play_top < plat_bottom && play_left < plat_right && play_left > plat_mid_x { //right
            self.view.set_x(plat_right);
            temp_jump_possib = false;
        } else if play_bottom > plat_top && play_top < plat_bottom && play_right > plat_left && play_right < plat_mid_x { //left
            self.view.set_x(plat_left-self.view.size().x);
            temp_jump_possib = false;
        }

        if play_bottom > 960.0 {
            self.reset_pos();
            self.allpurpose_hurt().await;
        }

        if temp_jump_possib {
            self.jump_posib = true;
        }
    }

    pub async fn automove(&mut self) {
        if self.player == "p1".to_string() {
            if is_key_down(KeyCode::W) && self.jump_posib{
                self.jump()
            }
            if is_key_down(KeyCode::A) {
                self.move_left().await;
            }

            if is_key_down(KeyCode::D) {
                self.move_right().await;
            }

            if is_key_down(KeyCode::Space) {
                self.attack().await;
            }

        } else {
            if is_key_down(KeyCode::Up) && self.jump_posib{
                self.jump()
            }
            if is_key_down(KeyCode::Left) {
                self.move_left().await;
            }

            if is_key_down(KeyCode::Right) {
                self.move_right().await;
            }

            if is_key_down(KeyCode::M) {
                self.attack().await;
            }
        }
        
        if get_time() - self.last_attack_time > 0.5 {
            self.attack = false;
        }

        self.framemove();
    }

    pub fn get_attack_state(&self) -> bool {
        self.attack
    }
}

//1280x960