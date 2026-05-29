/*
By: C.O.M Lab Studios
Date: 2026-04-15
Program Details: Main screen manager
*/

mod classes;
mod maps;
mod ui;
mod screens;
mod utils;

use macroquad::prelude::*;
use crate::screens::login::LoginRecord;
use crate::utils::preload_image::TextureManager;
use crate::utils::update_player_dbs::update_player_dbs;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::utils::virtual_screen::VirtualScreen;



/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "poultry_wars".to_string(),
        window_width: 1280,
        window_height: 960,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tm = TextureManager::new();
    let vs = VirtualScreen::new();
let all_assets = vec![
        "assets/class_images/chicken/p1/death.png",
        "assets/class_images/chicken/p1/attack_l.png",
        "assets/class_images/chicken/p1/attack_r.png",
        "assets/class_images/chicken/p1/idle_r.png",
        "assets/class_images/chicken/p1/idle_l.png",
        "assets/class_images/chicken/p2/death.png",
        "assets/class_images/chicken/p2/attack_r.png",
        "assets/class_images/chicken/p2/attack_l.png",
        "assets/class_images/chicken/p2/idle_r.png",
        "assets/class_images/chicken/p2/idle_l.png",
        "assets/class_images/turkey/p1/attack_r.png",
        "assets/class_images/turkey/p1/death.png",
        "assets/class_images/turkey/p1/idle_r.png",
        "assets/class_images/turkey/p1/attack_l.png",
        "assets/class_images/turkey/p1/idle_l.png",
        "assets/class_images/turkey/p2/attack_r.png",
        "assets/class_images/turkey/p2/death.png",
        "assets/class_images/turkey/p2/idle_r.png",
        "assets/class_images/turkey/p2/attack_l.png",
        "assets/class_images/turkey/p2/idle_l.png",
        "assets/health/stocks/1.png",
        "assets/health/stocks/2.png",
        "assets/health/stocks/3.png",
        "assets/health/chicken/p1/health0.png",
        "assets/health/chicken/p1/health1.png",
        "assets/health/chicken/p1/health10.png",
        "assets/health/chicken/p1/health2.png",
        "assets/health/chicken/p1/health3.png",
        "assets/health/chicken/p1/health4.png",
        "assets/health/chicken/p1/health5.png",
        "assets/health/chicken/p1/health6.png",
        "assets/health/chicken/p1/health7.png",
        "assets/health/chicken/p1/health8.png",
        "assets/health/chicken/p1/health9.png",
        "assets/health/chicken/p2/health0.png",
        "assets/health/chicken/p2/health1.png",
        "assets/health/chicken/p2/health10.png",
        "assets/health/chicken/p2/health2.png",
        "assets/health/chicken/p2/health3.png",
        "assets/health/chicken/p2/health4.png",
        "assets/health/chicken/p2/health5.png",
        "assets/health/chicken/p2/health6.png",
        "assets/health/chicken/p2/health7.png",
        "assets/health/chicken/p2/health8.png",
        "assets/health/chicken/p2/health9.png",
        "assets/health/turkey/p1/health0.png",
        "assets/health/turkey/p1/health1.png",
        "assets/health/turkey/p1/health10.png",
        "assets/health/turkey/p1/health2.png",
        "assets/health/turkey/p1/health3.png",
        "assets/health/turkey/p1/health4.png",
        "assets/health/turkey/p1/health5.png",
        "assets/health/turkey/p1/health6.png",
        "assets/health/turkey/p1/health7.png",
        "assets/health/turkey/p1/health8.png",
        "assets/health/turkey/p1/health9.png",
        "assets/health/turkey/p2/health0.png",
        "assets/health/turkey/p2/health1.png",
        "assets/health/turkey/p2/health10.png",
        "assets/health/turkey/p2/health2.png",
        "assets/health/turkey/p2/health3.png",
        "assets/health/turkey/p2/health4.png",
        "assets/health/turkey/p2/health5.png",
        "assets/health/turkey/p2/health6.png",
        "assets/health/turkey/p2/health7.png",
        "assets/health/turkey/p2/health8.png",
        "assets/health/turkey/p2/health9.png",
        "assets/maps/map_1/background.png",
        "assets/maps/map_1/miniplatform.png",
        "assets/maps/map_1/platform.png",
        "assets/maps/map_2/citymap.png",
        "assets/maps/map_2/left_floor.png",
        "assets/maps/map_2/right_floor.png",
        "assets/maps/map_2/cranebar.png",
        "assets/maps/map_2/cranechain.png",
        "assets/maps/map_2/airballoon.png",
        "assets/rank_images/1.png",
        "assets/rank_images/2.png",
        "assets/rank_images/3.png",
        "assets/rank_images/4.png",
        "assets/rank_images/5.png",
        "assets/welcome_screen/mainscreenbg.png",
        "assets/welcome_screen/welcome_logo.png",
        "assets/selection/ready.png",
        "assets/win_screen/winscreen.png",
        "assets/win_screen/trophy.png"
    ];
    tm.preload_with_loading_screen(&all_assets, None).await;


    let mut player_info: Vec<LoginRecord> = screens::login::run(&vm).await;
    let mut last_switch = get_time() - 0.02;
    let mut current_screen = "welcome".to_string();
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "welcome" => screens::welcome::run(&tm, &vm).await,
                "guide" => screens::guide::run(&vm).await,
                "game" => {
                    let game_info = screens::selection::run(&tm, &player_info[0].username, &player_info[1].username, &vm).await;
                    let winner = screens::game::run(&tm, game_info, &vm).await;
                    let retscreen = screens::win::run(&tm, &player_info[0], &player_info[1], winner, &vm).await;
                    player_info[0] = retscreen[0].clone();
                    player_info[1] = retscreen[1].clone();
                    let res = update_player_dbs(vec![player_info[0].clone(), player_info[1].clone()]).await;
                    if let Err(e) = res {
                        println!("Error updating player records: {}", e);
                    }
                    retscreen[2].username.clone()
                },
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}

#[wasm_bindgen]
extern "C" {}