/*
By: Mujibullah, edited by: Oliver
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
    let mut map_selected = false;

    let btn_class_chicken_1 = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Chicken",
        BLUE,
        GREEN,
        30
    );
    let btn_class_turkey_1 = TextButton::new(
        100.0,
        300.0,
        200.0,
        60.0,
        "Turkey",
        BLUE,
        GREEN,
        30
    );
    let btn_class_chicken_2 = TextButton::new(
        500.0,
        200.0,
        200.0,
        60.0,
        "Chicken",
        BLUE,
        GREEN,
        30
    );
    let btn_class_turkey_2 = TextButton::new(
        500.0,
        300.0,
        200.0,
        60.0,
        "Turkey",
        BLUE,
        GREEN,
        30
    );
    let btn_map_dirt = TextButton::new(
        900.0,
        200.0,
        200.0,
        60.0,
        "Dirt",
        BLUE,
        GREEN,
        30
    );
    let btn_p1_rdy = TextButton::new(
        100.0,
        400.0,
        200.0,
        60.0,
        "Ready",
        BLACK,
        GREEN,
        30
    );
    let btn_p2_rdy = TextButton::new(
        500.0,
        400.0,
        200.0,
        60.0,
        "Ready",
        BLACK,
        GREEN,
        30
    );
    let btn_map_select = TextButton::new(
        900.0,
        400.0,
        200.0,
        60.0,
        "Select",
        BLACK,
        GREEN,
        30
    );
    let btn_start = TextButton::new(
        1000.0,
        700.0,
        200.0,
        60.0,
        "Start",
        BLACK,
        GREEN,
        30
    );

    let lbl_p1 = Label::new(p1_user, 100.0, 150.0, 45);
    let lbl_p2 = Label::new(p2_user, 500.0, 150.0, 45);
    let lbl_map = Label::new("Map", 900.0, 150.0, 45);

    let lbl_p1_ready = Label::new(p1_user, 100.0, 700.0, 45);
    let lbl_p2_ready = Label::new(p2_user, 100.0, 800.0, 45);
    let lbl_map_selected = Label::new("Map", 100.0, 900.0, 45);

    let mut img_p1_char = StillImage::new("", 64.0, 64.0, 300.0, 650.0, true, 1.0).await;
    let mut img_p2_char = StillImage::new("", 64.0, 64.0, 300.0, 750.0, true, 1.0).await;
    let mut img_map = StillImage::new("", 64.0, 64.0, 300.0, 850.0, true, 1.0).await;
    let mut img_p1_rdy = StillImage::new("", 32.0, 32.0, 316.0, 666.0, true, 1.0).await;
    let mut img_p2_rdy = StillImage::new("", 32.0, 32.0, 316.0, 766.0, true, 1.0).await;
    let mut img_map_rdy = StillImage::new("", 32.0, 32.0, 316.0, 866.0, true, 1.0).await;
    img_p1_rdy.set_preload(tm.get_preload("assets/selection/ready.png").unwrap());
    img_p2_rdy.set_preload(tm.get_preload("assets/selection/ready.png").unwrap());
    img_map_rdy.set_preload(tm.get_preload("assets/selection/ready.png").unwrap());

    let mut player_info = vec!["u".to_string(), "u".to_string(), "u".to_string()]; //p1, p2, map
    

loop{
    clear_background(BEIGE);
    draw_line(400.0, 100.0, 400.0, 500.0, 4.0, BLACK);
    draw_line(750.0, 100.0, 750.0, 500.0, 4.0, BLACK);
    lbl_p1.draw();
    lbl_p2.draw();
    lbl_map.draw();
    lbl_p1_ready.draw();
    lbl_p2_ready.draw();
    lbl_map_selected.draw();
    img_p1_char.draw();
    img_p2_char.draw();
    img_map.draw();

    if p1_ready {
        img_p1_rdy.draw();
    }
    if p2_ready {
        img_p2_rdy.draw();
    }
    if map_selected {
        img_map_rdy.draw();
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
    if player_info[2] == "dirt".to_string() {
        img_map.set_preload(tm.get_preload("assets/maps/map_1/background.png").unwrap());
    }


    if is_key_pressed(KeyCode::K) {
        return vec!["chicken".to_string(), "turkey".to_string(), "dirt".to_string()];
    }

    if btn_p1_rdy.click() {
        if player_info[0] != "u".to_string() {
            if p1_ready {
            p1_ready = false;
            } else {
                p1_ready = true;
            }
        }
    }

    if btn_p2_rdy.click() {
        if player_info[1] != "u".to_string() {
            if p2_ready {
                p2_ready = false;
            } else {
                p2_ready = true;
            }
        }
    }

    if btn_map_select.click() {
        if player_info[2] != "u".to_string() {
            if map_selected {
                map_selected = false;
            } else {
                map_selected = true;
            }
        }
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
    if btn_map_dirt.click() {
        player_info[2] = "dirt".to_string();
    }

    if p1_ready && p2_ready && map_selected {
        if btn_start.click() {
            return player_info;
        }
    }
    
    next_frame().await;
}

}

