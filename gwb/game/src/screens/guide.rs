/*
By Mujibullah
Date: 04/05/2026
Describtion: A guide screen helping the user to navigate through the game and movement
 */

use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::utils::scale::use_virtual_resolution;


pub async fn run() -> String{
    let btn_back = TextButton::new(1150.0, 20.0, 100.0, 50.0, "Back", BLACK, BLUE, 35);
     let mut lbl_guide = Label::new(
        "                Greeting and Salutations dear players!
Welcome to Poultry WarsThis is a 1v1 game where you,
our dear players would be playing against another player as CHICKENS.
In the game you have keys as                             or TURKEYS",
        50.0, 50.0, 40);
    let mut lbl_movement = Label::new(
        "Player 1		                            Player 2
A- move left	                        - move left
D- move right	                       - move right
W- to jump		                         - to jump
Spacebar - to attack 	               M - to attack


You will be able to store your data such as:
Your Rank,Username,Password",
        50.0, 250.0, 35);
    let mut lbl_special = Label::new ("Chickens and Turkeys have their own special movement:
    For Chickens it's a slam down when you click the key s
    For Turkeys it's a dash when you click left or right arrow with the arrow down key.",50.0, 640.0, 30);
    lbl_guide.with_colors(BLACK, Some(LIGHTGRAY));
    lbl_movement.with_colors(BLACK, Some(LIGHTGRAY));
    lbl_special.with_colors(BLACK, Some(LIGHTGRAY));

loop{
    use_virtual_resolution(1280.0, 960.0);
    clear_background(BEIGE);
    draw_grid(50.0, BLACK);
    lbl_guide.draw();
    lbl_movement.draw();
    lbl_special.draw();
    if btn_back.click(){
        return "welcome".to_string();
    }
    next_frame().await;
}
}
