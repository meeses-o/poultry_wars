//Created by Oliver
//Apr 24th 2026
//Test screen

use macroquad::prelude::*;
use crate::ui::text_button::TextButton;

pub async fn run() -> String{
    let btn_exit = TextButton::new(0.0, 0.0, 100.0, 100.0, "Exit", BLACK, RED, 30);
    

    loop {
        clear_background(BEIGE);
        if btn_exit.click() {
            return "exit".to_string();
        }
        if is_key_down(KeyCode::Backspace){
            return "game".to_string();
        }
        next_frame().await
    }
}