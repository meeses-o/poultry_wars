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
    let lbl_guide = Label::new(20.0, 80.0, 100.0, 50.0, "Greeting and Salutations dear players!
Welcome to Poultry Wars
This is a 1v1 game where you, our dear players would be playing against another player as CHICKENS!
In the game you have keys as
Player 1		Player 2
A- move left	   - move left
D- move right	   - move right
W- to jump		   - to jump
    Spacebar - to attack 	 Enter - to attack







You will be able to store your data such as: 
Your Rank,
Username,
Password
");
loop{
    if btn_exit.click() {
        break;
    }
}
}
