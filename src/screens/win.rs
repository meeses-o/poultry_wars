use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;

pub async fn run(user1: String, user2: String, winner: bool) -> String {
    let mut lbl_win_txt = String::new();
    let mut win_img_path = String::new();
    if winner {
        lbl_win_txt = format!("{} WINS", user1);
        let win_img_path = "assets/class_images/chicken/p1/idle_r.png";
    } else {
        lbl_win_txt = format!("{} WINS", user2);
        let win_img_path = "assets/class_images/chicken/p2/idle_r.png";
    }
    let lbl_winner = Label::new(lbl_win_txt,100.0,200.0,25,);
    let win_img_path = StillImage::new(win_img_path, 100.0, 100.0, 200.0, 200.0);
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
        clear_background(BLACK);

        lbl_winner.draw();
        win_img_path.draw(100.0, 100.0, 200.0, 200.0);
        next_frame().await
    }
}


