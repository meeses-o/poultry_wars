/*
By Mujibullah
Date: 04/05/2026
Describtion: A guide screen helping the user to navigate through the game and movement
 */

use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;

pub async fn run(){
    let btn_exit = TextButton::new(20.0, 20.0, 100.0, 50.0, "Exit");
    let lbl_guide = Label::new(20.0, 80.0, 100.0, 50.0, "Use arrow keys to move");
loop{
    if btn_exit.click() {
        break;
    }
}
}
