/*
By Mujibullah
Date: 04/05/2026
Describtion: A guide screen helping the user to navigate through the game and movement
 */

use macroquad::prelude::*;
use crate::ui::grid::{ draw_grid };
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;

pub async fn run() -> String{
    let btn_exit = TextButton::new(20.0, 20.0, 100.0, 50.0, "Exit", BLACK, BLUE, 15);
    let btn_login = TextButton::new(100.0, 20.0, 100.0, 50.0, "Login", BLACK, BLUE, 15);
    let lbl_guide = Label::new(
        "Greeting and Salutations dear players!\n
        Welcome to Poultry WarsThis is a 1v1 game where you, our dear players would be playing against another player as CHICKENS!In the game you have keys as\n
        Player 1		Player 2\n
        A- move left	   - move left\n
        D- move right	   - move right\n
        W- to jump		   - to jump\n
        Spacebar - to attack 	 Enter - to attac\n
        You will be able to store your data such as:\n
        Your Rank,Username,Password",
        20.0, 80.0, 35);
loop{
    clear_background(BEIGE);
    draw_grid(50.0, BLACK);
    lbl_guide.draw();
    if btn_login.click(){
        return "login".to_string();
    }
    if btn_exit.click(){
        return "exit".to_string();
    }
    next_frame().await;

}
}
