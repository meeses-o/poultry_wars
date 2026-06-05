//Created by Oliver
//Apr 24th 2026
//Test screen

use macroquad::prelude::*;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::scale::use_virtual_resolution;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};


pub async fn run(tm: &TextureManager) -> String{
    let btn_game = TextButton::new(
        430.0,
        600.0,
        200.0,
        60.0,
        "PLAY",
        BLUE,
        GREEN,
        30
    );
    let btn_guide = TextButton::new(
        650.0,
        600.0,
        200.0,
        60.0,
        "GUIDE",
        BLUE,
        GREEN,
        30
    );
    let mut img_logo = StillImage::new("", 500.0, 400.0, 330.0, 0.0, false, 10.0).await;
    img_logo.set_preload(tm.get_preload("assets/welcome_screen/welcome_logo.png").unwrap());

    let mut img_bkg = StillImage::new("", 1280.0, 960.0, 0.0, 0.0, true, 1.0).await;
    img_bkg.set_preload(tm.get_preload("assets/welcome_screen/mainscreenbg.png").unwrap());

    let music = load_sound("assets/pwars_theme.ogg").await.unwrap();
    play_sound(
        &music,
        PlaySoundParams {
            volume: 0.5,
            looped: true
        },
    );
    loop {
        use_virtual_resolution(1280.0, 960.0);
        clear_background(BEIGE);
        img_bkg.draw();
        img_logo.draw();
        if btn_game.click() {
            return "game".to_string();
        }
        if btn_guide.click() {
            return "guide".to_string();
        }
        img_logo.set_zoom(((1.0*get_time().sin())+9.0) as f32);
        next_frame().await
    }
}

//1280x960