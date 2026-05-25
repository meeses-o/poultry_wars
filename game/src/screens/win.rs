use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;
use crate::utils::preload_image::TextureManager;
use crate::screens::login::LoginRecord;
use crate::utils::rank_calculator::calc_rank;

//winner = 1; p1, etc.
pub async fn run(tm: &TextureManager, user1: &LoginRecord, user2: &LoginRecord, winner: bool) -> Vec<LoginRecord> {
    let radius: f32 = 300.0;
    let mut angle: f32 = 0.0;
    let mut orbit_x = 0.0;
    let mut orbit_y = 0.0;
    let center_x = 640.0;
    let center_y = 480.0;
    let mut lbl_win_txt = String::new();
    let mut win_img_path = String::new();
    let mut img_background_path= String::new();
    let mut img_trophy_path = String::new();
    let mut user1_local = user1.clone();
    let mut user2_local = user2.clone();
    let retscreen = LoginRecord::new(0, "".to_string(), "".to_string(), "".to_string(), "".to_string());
    
    img_background_path = "assets/win_screen/winscreen.png".to_string();
    if winner {
        lbl_win_txt = format!("{} WINS", user1_local.username);
        win_img_path = "assets/class_images/chicken/p1/idle_r.png".to_string();
        let rank = user1_local.rank.parse::<i32>().unwrap() + 1;
        user1_local.rank = format!("{}", rank);
        let money = user1_local.money.parse::<i32>().unwrap() + 50;
        user1_local.money = format!("{}", money);
        img_trophy_path = format!("assets/rank_images/{}.png", calc_rank(&user1_local.rank))
    } else {
        lbl_win_txt = format!("{} WINS", user2_local.username);
        win_img_path = "assets/class_images/chicken/p2/idle_r.png".to_string();
        let rank = user2_local.rank.parse::<i32>().unwrap() + 1;
        user2_local.rank = format!("{}", rank);
        let money = user2_local.money.parse::<i32>().unwrap() + 50;
        user2_local.money = format!("{}", money);
        img_trophy_path = format!("assets/rank_images/{}.png", calc_rank(&user2_local.rank))
    }
    let mut ret_vec = vec![user1_local, user2_local, retscreen];

    let lbl_winner = Label::new(lbl_win_txt,640.0,100.0,25);
    let mut win_img = StillImage::new("", 640.0, 400.0, 300.0, 300.0, true, 1.0).await;
    let mut img_background = StillImage::new("", screen_width(), screen_height(), 0.0, 0.0, true, 1.0).await;
    img_background.set_preload(tm.get_preload(&img_background_path).unwrap());
    let mut img_trophy = StillImage::new("", 100.0, 150.0, 590.0, 775.0, true, 1.0).await;
    img_trophy.set_preload(tm.get_preload(&img_trophy_path).unwrap());
    win_img.set_preload(tm.get_preload(&win_img_path).unwrap());
    let btn_playagain = TextButton::new(
        400.0,
        100.0,
        200.0,
        60.0,
        "BACK TO MENU",
        BLUE,
        GREEN,
        30
    );
    let btn_exit = TextButton::new(
        800.0,
        100.0,
        200.0,
        60.0,
        "EXIT GAME",
        BLUE,
        GREEN,
        30
    );

    loop {
        img_background.draw();
        if btn_playagain.click(){
            ret_vec[2].username = "game".to_string();
            return ret_vec;
        }
        if btn_exit.click(){
            ret_vec[2].username = "exit".to_string();
            return ret_vec;
        }
        img_trophy.set_angle(angle);
        angle -= 0.001;
        img_trophy.set_position(Vec2::new(orbit_x, orbit_y));
        orbit_x = center_x + angle.cos() * radius;
        orbit_y = center_y + angle.sin() * radius;
        lbl_winner.draw();
        win_img.draw();
        img_trophy.draw();
        next_frame().await
    }
}