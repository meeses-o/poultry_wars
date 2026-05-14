/*
By: Mujibullah
Date: 05/13/2026
Details: A selection screen where players can select from a list of options of their classes and which map they want to play on. 
 */
use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: &TextureManager, p1_user: &String, p2_user: &String) -> Vec<String> {
    let mut p1_ready = false;
    let mut p2_ready = false;

    let btn_class_chicken_1 = TextButton::new(
        100.0,
        300.0,
        200.0,
        60.0,
        "Chicken",
        BLUE,
        GREEN,
        30
    );
    let btn_class_turkey_1 = TextButton::new(
        100.0,
        400.0,
        200.0,
        60.0,
        "Turkey",
        BLUE,
        GREEN,
        30
    );
    let btn_class_chicken_2 = TextButton::new(
        500.0,
        300.0,
        200.0,
        60.0,
        "Chicken",
        BLUE,
        GREEN,
        30
    );
    let btn_class_turkey_2 = TextButton::new(
        500.0,
        400.0,
        200.0,
        60.0,
        "Turkey",
        BLUE,
        GREEN,
        30
    );
    let btn_map_1 = TextButton::new(
        1000.0,
        300.0,
        200.0,
        60.0,
        "First Map",
        BLUE,
        GREEN,
        30
    );
    let lbl_p1 = Label::new(p1_user, 100.0, 150.0, 45);
    let lbl_p2 = Label::new(p2_user, 500.0, 150.0, 45);

    let lbl_p1_ready = Label::new(p1_user, 100.0, 700.0, 45);
    let lbl_p2_ready = Label::new(p2_user, 100.0, 800.0, 45);

    let mut img_p1_char = StillImage::new("", 64.0, 64.0, 300.0, 650.0, true, 1.0).await;
    let mut img_p2_char = StillImage::new("", 64.0, 64.0, 300.0, 750.0, true, 1.0).await;
    let mut img_p1_rdy = StillImage::new("", 32.0, 32.0, 316.0, 666.0, true, 1.0).await;
    let mut img_p2_rdy = StillImage::new("", 32.0, 32.0, 316.0, 766.0, true, 1.0).await;
    img_p1_rdy.set_preload(tm.get_preload("assets/selection/ready.png").unwrap());
    img_p2_rdy.set_preload(tm.get_preload("assets/selection/ready.png").unwrap());

    let mut player_info = vec!["".to_string(), "".to_string(), "".to_string()];
    

loop{
    clear_background(BEIGE);
    // draw_grid(50.0, BLACK);
    draw_line(400.0, 100.0, 400.0, 500.0, 4.0, BLACK);
    draw_line(750.0, 100.0, 750.0, 500.0, 4.0, BLACK);
    lbl_p1.draw();
    lbl_p2.draw();
    lbl_p1_ready.draw();
    lbl_p2_ready.draw();
    img_p1_char.draw();
    img_p2_char.draw();

    if p1_ready {
        img_p1_rdy.draw();
    }
    if p2_ready {
        img_p2_rdy.draw();
    }

    if player_info[0] == "chicken".to_string() {
        img_p1_char.set_preload(tm.get_preload("assets/class_images/chicken/p1/idle_r.png").unwrap());
    } else if player_info[0] == "turkey".to_string() {
        img_p1_char.set_preload(tm.get_preload("assets/class_images/turkey/p1/idle_r.png").unwrap());
    }
    if player_info[1] == "chicken".to_string() {
        img_p2_char.set_preload(tm.get_preload("assets/class_images/chicken/p2/idle_r.png").unwrap());
    } else if player_info[1] == "turkey".to_string() {
        img_p2_char.set_preload(tm.get_preload("assets/class_images/turkey/p2/idle_r.png").unwrap());
    }

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

