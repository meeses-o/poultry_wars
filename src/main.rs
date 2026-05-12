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
    let player_info: Vec<LoginRecord> = screens::login::run().await;
    let mut last_switch = get_time() - 0.02;
    let mut current_screen = "screen1".to_string();
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "screen1" => screens::screen1::run().await,
                "game" => {
                    let winner = screens::game::run().await;
                    let retscreen = screens::win::run(&player_info[0].username, &player_info[1].username, winner).await;
                    retscreen
                }
                "guide" => screens::guide::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
