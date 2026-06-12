//Created by Oliver
//May 13th 2026
//Turkey player class

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub struct Turkey{
    tm: TextureManager,
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
    side: bool,                 //left = false, right = true
    attack_clock: bool,
    attack_cooldown: f32,
    last_dash_time: f64,
    dash_possible: bool,
    hit: bool,
    hit_since: f64,
    hitmarker: StillImage
}

impl Turkey {
    pub async fn new(tm:&TextureManager,x:f32, y:f32, player: String, start_pos: Vec2) -> Self {
        if player == "p1".to_string() {
            Turkey {
                tm: tm.clone(),
                view: StillImage::new("", 50.0, 100.0, x, y, true, 1.0).await,
                health: 10,
                stocks: 3,
                health_bar: StillImage::new("", 320.0, 200.0, 650.0, -38.0, true, 1.0).await,
                stock_counter: StillImage::new("", 100.0, 100.0, 650.0, 50.0, true, 1.0).await,
                player: player,
                speed: vec2(0.0,0.0),
                accell: vec2(0.0,0.125),
                jump_posib: true,
                start_pos: start_pos,
                attack: false,
                last_attack_time: -0.5,
                side: false,
                attack_clock: false,
                attack_cooldown: 1.0,
                last_dash_time: 0.0,
                dash_possible: true,
                hit: false,
                hit_since: 0.0,
                hitmarker: StillImage::new("", 100.0, 100.0, 0.0, 0.0, true, 1.0).await
            }  
        } else {
            Turkey {
                tm: tm.clone(),
                view: StillImage::new("", 50.0, 100.0, x, y, true, 1.0).await,
                health: 10,
                stocks: 3,
                health_bar: StillImage::new("", 320.0, 200.0, 320.0, -34.0, true, 1.0).await,
                stock_counter: StillImage::new("", 100.0, 100.0, 320.0, 50.0, true, 1.0).await,
                player: player,
                speed: vec2(0.0,0.0),
                accell: vec2(0.0,0.125),
                jump_posib: true,
                start_pos: start_pos,
                attack: false,
                last_attack_time: -0.5,
                side: true,
                attack_clock: false,
                attack_cooldown: 1.0,
                last_dash_time: 0.0,
                dash_possible: true,
                hit: false,
                hit_since: 0.0,
                hitmarker: StillImage::new("", 100.0, 100.0, 0.0, 0.0, true, 1.0).await
            }
        }
    }

    pub fn init(&mut self) {
        self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/idle_r.png", self.player)).unwrap());
        self.health_bar.set_preload(self.tm.get_preload(&format!("assets/health/turkey/{}/health10.png", self.player)).unwrap());
        self.stock_counter.set_preload(self.tm.get_preload("assets/health/stocks/3.png").unwrap());
    }

    pub fn draw(&self) {
        self.health_bar.draw();
        self.stock_counter.draw();
        self.hitmarker.draw();
        self.view.draw();
    }

    pub fn get_view(&self) -> &StillImage {
        &self.view
    }

    pub fn framemove(&mut self) {
        let dt_scale = get_frame_time() * 166.0;

        let mut topspeedx = 4.5;
        if !self.dash_possible {
            topspeedx = 10.0;
        }
        let topspeedy = 5.0;

        self.speed += self.accell * dt_scale;

        let decel = 0.5 * dt_scale;
        if self.speed.x > decel {
            self.speed.x -= decel;
        } else if self.speed.x < -decel {
            self.speed.x += decel;
        } else {
            self.speed.x = 0.0;
        }

        if self.speed.x > topspeedx {
            self.speed.x = topspeedx;
        } else if self.speed.x < -topspeedx {
            self.speed.x = -topspeedx;
        }

        if self.speed.y > topspeedy {
            self.speed.y = topspeedy;
        }

        let mut pos = self.view.get_pos() + (self.speed * dt_scale);
        self.view.set_position(pos);

        if self.hit {
            if get_time() - self.hit_since > 0.08 {
                pos.x -= 25.0;
                self.hitmarker.set_position(pos);
                self.hitmarker.set_preload(self.tm.get_preload("assets/extra_effects/hit_effect.png").unwrap());
            } if get_time()- self.hit_since > 0.4{
                // self.hit = false;
                self.hitmarker.clear();
            }
        }
    }

    fn move_left(&mut self) {
        let dt_scale = get_frame_time() * 166.0;
        self.speed.x -= 0.6 * dt_scale;
        self.side = false;
        if !self.attack {
            self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/idle_l.png", self.player)).unwrap());
        } else {
            self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/attack_l.png", self.player)).unwrap());
        }
    }

    fn move_right(&mut self) {
        let dt_scale = get_frame_time() * 166.0;
        self.speed.x += 0.6 * dt_scale;
        self.side = true;
        if !self.attack {
            self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/idle_r.png", self.player)).unwrap());
        } else {
            self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/attack_r.png", self.player)).unwrap());
        }
    }

    pub fn hit_animate(&mut self, op_side: bool) {
        self.hit = true;
        self.hit_since = get_time();
        self.speed.y -= 3.0;
        if op_side {
            self.speed.x += 10.0;
        } else {
            self.speed.x -= 10.0;
        }
    }

    pub fn get_side(&self) -> bool {
        self.side
    }

    fn jump(&mut self) {
        self.speed.y = -6.0;
        self.jump_posib = false;
    }

    fn dash(&mut self) {
        if (get_time() - self.last_dash_time > 1.2) && self.dash_possible {
            self.dash_possible = false;
            self.last_dash_time = get_time();
            if self.side {
                self.speed.x = 10.0;
            } else {
                self.speed.x = -10.0;
            }
        }
    }

    fn attack(&mut self) {
        let time = get_time();
        if time - self.last_attack_time > self.attack_cooldown as f64 {
            self.attack = true;
            self.attack_clock = true;
            self.last_attack_time = time;
        }
    }

    fn idle(&mut self) {
        if self.side {
            if !self.attack {
                self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/idle_r.png", self.player)).unwrap());
            } else {
                self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/attack_r.png", self.player)).unwrap());
            }
        } else {
            if !self.attack {
            self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/idle_l.png", self.player)).unwrap());
            } else {
                self.view.set_preload(self.tm.get_preload(&format!("assets/class_images/turkey/{}/attack_l.png", self.player)).unwrap());
            }
        }
    }

    pub fn reset_pos(&mut self) {
        self.view.set_position(self.start_pos);
    }

    fn hurt_single(&mut self) {
        if self.health < 1 && self.stocks > 1 {
            self.stocks -= 1;
            self.health = 10;
            self.view.set_position(self.start_pos);
        } else if self.health > 1 {
            self.health -= 1;
        } else {
            self.health = 0;
        }
        self.health_bar.set_preload(self.tm.get_preload(&format!("assets/health/turkey/{}/health{}.png", self.player, self.health)).unwrap());
        self.stock_counter.set_preload(self.tm.get_preload(&format!("assets/health/stocks/{}.png", self.stocks)).unwrap());
    }

    pub fn allpurpose_hurt(&mut self, damage: i32) {
        for _ in 0..damage {
            self.hurt_single();
        }
    }

    pub fn check_full_dead(&self) -> bool {
        if self.health == 0 && self.stocks == 1 {
            return true;
        } else {
            return false;
        }
    }

    pub fn check_floor_at(&mut self, floordims: Vec<Vec2>) {
        let dt_scale = get_frame_time() * 166.0;

        // current player dims
        let play_top = self.view.get_y();
        let mut play_bottom = self.view.get_y() + self.view.size().y;
        let play_left = self.view.get_x();
        let play_right = self.view.get_x() + self.view.size().x;

        // Plat dims
        let plat_top = floordims[0].y;
        let plat_bottom = floordims[0].y + floordims[1].y;
        let plat_left = floordims[0].x;
        let plat_right = floordims[0].x + floordims[1].x;

        let intersect_x = play_left < plat_right && play_right > plat_left;
        let intersect_y = play_top < plat_bottom && play_bottom > plat_top;

        let mut temp_jump_possib = false;

        if intersect_x && intersect_y {
            //prev player pos
            let prev_play_bottom = play_bottom - (self.speed.y * dt_scale);
            let prev_play_top = play_top - (self.speed.y * dt_scale);
            let prev_play_left = play_left - (self.speed.x * dt_scale);
            let prev_play_right = play_right - (self.speed.x * dt_scale);

            if prev_play_bottom <= plat_top + 0.01 { 
                self.view.set_y(plat_top - self.view.size().y);
                self.speed.y = 0.0;
                temp_jump_possib = true;
            } else if prev_play_top >= plat_bottom - 0.01 {
                self.view.set_y(plat_bottom);
                self.speed.y = 0.0;
            } else if prev_play_right <= plat_left + 0.01 {
                self.view.set_x(plat_left - self.view.size().x);
                self.speed.x = 0.0;
            } else if prev_play_left >= plat_right - 0.01 {
                self.view.set_x(plat_right);
                self.speed.x = 0.0;
            }
        }

        play_bottom = self.view.get_y() + self.view.size().y;

        if play_bottom > 960.0 {
            self.reset_pos();
            self.allpurpose_hurt(1);
        }

        if temp_jump_possib {
            self.jump_posib = true;
            if !self.dash_possible {
                self.dash_possible = true;
            }
        }
    }

    pub fn automove(&mut self) {
        if self.attack_clock {
            self.attack_clock = false;
            
        }

        if self.player == "p1".to_string() {
            let mut keypress = false;

            if is_key_down(KeyCode::W) && self.jump_posib{
                self.jump();
                keypress = true;
            }
            if is_key_down(KeyCode::A) {
                self.move_left();
                keypress = true;
            }

            if is_key_down(KeyCode::D) {
                self.move_right();
                keypress = true;
            }

            if is_key_down(KeyCode::Space) {
                self.attack();
            }

            if is_key_down(KeyCode::S) {
                self.dash();
                keypress = true;
            }

            if !keypress {
                self.idle();
            }

        } else {
            let mut keypress = false;

            if is_key_down(KeyCode::Up) && self.jump_posib{
                self.jump();
                keypress = true;
            }
            if is_key_down(KeyCode::Left) {
                self.move_left();
                keypress = true;
            }

            if is_key_down(KeyCode::Right) {
                self.move_right();
                keypress = true;
            }

            if is_key_down(KeyCode::M) {
                self.attack();
            }

            if is_key_down(KeyCode::Down) {
                self.dash();
                keypress = true;
            }

            if !keypress {
                self.idle();
            }
        }
        
        if get_time() - self.last_attack_time > self.attack_cooldown as f64 {
            self.attack = false;
        }

        self.framemove();
    }

    pub fn get_attack_state(&self) -> bool {
        self.attack_clock
    }

    pub fn get_attack_dmg(&self) -> i32 {
        2
    }
}