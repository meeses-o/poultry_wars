use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: &TextureManager, user1: &String, user2: &String, winner: bool) -> String {
    let mut lbl_win_txt = String::new();
    let mut win_img_path = String::new();
    if winner {
        lbl_win_txt = format!("{} WINS", user1);
        win_img_path = "assets/class_images/chicken/p1/idle_r.png".to_string();
    } else {
        lbl_win_txt = format!("{} WINS", user2);
        win_img_path = "assets/class_images/chicken/p2/idle_r.png".to_string();
    }
    let lbl_winner = Label::new(lbl_win_txt,100.0,200.0,25);
    let mut win_img = StillImage::new("", 100.0, 100.0, 200.0, 200.0, true, 1.0).await;
    win_img.set_preload(tm.get_preload(&win_img_path).unwrap());
    let btn_menu = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "BACK TO MENU",
        BLUE,
        GREEN,
        30
    );

    loop {
        clear_background(WHITE);
        if btn_menu.click(){
            return "guide".to_string();
        }
        lbl_winner.draw();
        win_img.draw();
        next_frame().await
    }
}