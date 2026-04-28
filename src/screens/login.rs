//Created by Cameron
//April 28th, 2026
//This file contains the code for the login screen

use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::textinput;

let login_button: TextButton = TextButton::new("Login", 100.0, 50.0);
let mut txt_username: = textinput::TextInput::new(200.0, 50.0);
let mut txt_password: = textinput::TextInput::new(200.0, 50.0);

pub fn login_screen() {
    clear_background(WHITE);

    //Draw the username and password text inputs
    txt_username.draw(50.0, 50.0);
    txt_password.draw(50.0, 150.0);

    //Draw the login button
    login_button.draw(50.0, 250.0);

    //Check for button click
    if login_button.is_clicked() {
        let username = txt_username.get_text();
        let password = txt_password.get_text();
    }
}

