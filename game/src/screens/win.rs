use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;
use crate::utils::preload_image::TextureManager;
use crate::screens::login::LoginRecord;

pub async fn run(tm: &TextureManager, user1: &LoginRecord, user2: &LoginRecord, winner: bool) -> Vec<LoginRecord> {
    let mut lbl_win_txt = String::new();
    let mut win_img_path = String::new();

    let mut user1_local = user1.clone();
    let mut user2_local = user2.clone();
    let mut retscreen = LoginRecord::new(0, "".to_string(), "".to_string(), "".to_string(), "".to_string());

    if winner {
        lbl_win_txt = format!("{} WINS", user1_local.username);
        win_img_path = "assets/class_images/chicken/p1/idle_r.png".to_string();
        let rank = user1_local.rank.parse::<i32>().unwrap() + 1;
        user1_local.rank = format!("{}", rank);
        let money = user1_local.money.parse::<i32>().unwrap() + 50;
        user1_local.money = format!("{}", money);
    } else {
        lbl_win_txt = format!("{} WINS", user2_local.username);
        win_img_path = "assets/class_images/chicken/p2/idle_r.png".to_string();
        let rank = user2_local.rank.parse::<i32>().unwrap() + 1;
        user2_local.rank = format!("{}", rank);
        let money = user2_local.money.parse::<i32>().unwrap() + 50;
        user2_local.money = format!("{}", money);
    }
    let mut ret_vec = vec![user1_local, user2_local, retscreen];

    let lbl_winner = Label::new(lbl_win_txt,640.0,100.0,25);
    let mut win_img = StillImage::new("", 640.0, 400.0, 300.0, 300.0, true, 1.0).await;
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
        clear_background(WHITE);
        if btn_playagain.click(){
            ret_vec[2].username = "game".to_string();
            return ret_vec;
        }
        if btn_exit.click(){
            ret_vec[2].username = "exit".to_string();
            return ret_vec;
        }
        lbl_winner.draw();
        win_img.draw();
        next_frame().await
    }
}