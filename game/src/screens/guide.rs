/*
By Mujibullah
Date: 04/05/2026
Describtion: A guide screen helping the user to navigate through the game and movement
 */

use macroquad::prelude::*;
// use crate::ui::grid::{ draw_grid };
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;

pub async fn run() -> String{
    let btn_back = TextButton::new(1150.0, 20.0, 100.0, 50.0, "Back", BLACK, BLUE, 35);
     let lbl_guide = Label::new(
        "                         Greeting and Salutations dear players!
Welcome to Poultry WarsThis is a 1v1 game where you,
our dear players would be playing against another player as CHICKENS.
In the game you have keys as",
        20.0, 50.0, 25);
    let lbl_movement = Label::new(
        "Player 1		                            Player 2
A- move left	                        - move left
D- move right	                       - move right
W- to jump		                         - to jump
Spacebar - to attack 	               M - to attack


You will be able to store your data such as:
Your Rank,Username,Password",
        20.0, 180.0, 25);

    //YOU MUST PRELOAD THESE.

    // let img_up = StillImage::new(
    //     "assets/arrow_up.png",
    //     100.0,  // width
    //     200.0,  // height
    //     550.0,  // x position
    //     380.0,   // y position
    //     true,   // Enable stretching
    //     1.0,    // Normal zoom (100%)
    // ).await;
    // let img_left = StillImage::new(
    //     "assets/arrow_left.png",
    //     100.0,  // width
    //     200.0,  // height
    //     550.0,  // x position
    //     400.0,   // y position
    //     true,   // Enable stretching
    //     1.0,    // Normal zoom (100%)
    // ).await;
    // let img_right = StillImage::new(
    //     "assets/arrow_right.png",
    //     100.0,  // width
    //     200.0,  // height
    //     550.0,  // x position
    //     430.0,   // y position
    //     true,   // Enable stretching
    //     1.0,    // Normal zoom (100%)
    // ).await;
    
loop{
    clear_background(BEIGE);
    // draw_grid(50.0, BLACK);
    lbl_guide.draw();
    lbl_movement.draw();
    if btn_back.click(){
        return "welcome".to_string();
    }
    next_frame().await;

}
}
