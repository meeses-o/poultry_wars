/*
By: Mujibullah
Date: 05/13/2026
Details: A selection screen where players can select from a list of options of their classes and which map they want to play on. 
 */
use macroquad::prelude::*;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: &TextureManager) -> Vec<String> {
let btn_class_chicken_1 = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Character: Chicken",
        BLUE,
        GREEN,
        30
    );
    let btn_class_turkey_1 = TextButton::new(
        100.0,
        300.0,
        200.0,
        60.0,
        "Character: Turkey",
        BLUE,
        GREEN,
        30
    );
    let btn_class_chicken_2 = TextButton::new(
        100.0,
        400.0,
        200.0,
        60.0,
        "Character: Chicken",
        BLUE,
        GREEN,
        30
    );
    let btn_class_turkey_2 = TextButton::new(
        100.0,
        500.0,
        200.0,
        60.0,
        "Character: Turkey",
        BLUE,
        GREEN,
        30
    );
    let btn_map_1 = TextButton::new(
        600.0,
        200.0,
        200.0,
        60.0,
        "First Map",
        BLUE,
        GREEN,
        30
    );

    let mut player_info = vec!["".to_string(), "".to_string(), "".to_string()];
    

loop{
    clear_background(BEIGE);
    draw_grid(50.0, BLACK);
    if is_key_pressed(KeyCode::K) {
        return vec!["chicken".to_string(), "turkey".to_string(), "map1".to_string()];
    }

   
    if btn_class_chicken_1.click() {
        player_info[0] = "chicken".to_string();
    }
    if btn_class_turkey_1.click() {
        player_info[0] = "turkey".to_string();
    }
    if btn_class_chicken_2.click() {
        player_info[1] = "chicken".to_string();
    }
    if btn_class_turkey_2.click() {
        player_info[1] = "turkey".to_string();
    }
    if btn_map_1.click() {
        player_info[2] = "map1".to_string();
    }
    // return player_info;

    
    next_frame().await;
}

}

