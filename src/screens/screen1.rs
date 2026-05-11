//Created by Oliver
//Apr 24th 2026
//Test screen

use macroquad::prelude::*;
use crate::ui::text_button::TextButton;

pub async fn run() -> String{
    let btn_exit = TextButton::new(0.0, 0.0, 100.0, 100.0, "Exit", BLACK, RED, 30);
    let btn_login = TextButton::new(
        700.0,
        200.0,
        200.0,
        60.0,
        "Login",
        BLUE,
        GREEN,
        30
    );
    let btn_game = TextButton::new(
        200.0,
        200.0,
        200.0,
        60.0,
        "Game",
        BLUE,
        GREEN,
        30
    );
    let btn_guide = TextButton::new(
        450.0,
        200.0,
        200.0,
        60.0,
        "Guide",
        BLUE,
        GREEN,
        30
    );



    

    loop {
        clear_background(BEIGE);
        if btn_exit.click() {
            return "exit".to_string();
        }
        if btn_login.click() {
            return "login".to_string()
        }
        if btn_game.click() {
            return "game".to_string();
        }
        if btn_guide.click() {
            return "guide".to_string();
        }
        next_frame().await
    }
}