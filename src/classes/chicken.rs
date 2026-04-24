// //Created by Oliver
// //Apr 24th 2026

// use macroquad::prelude::*;
// use crate::ui::still_image::StillImage;

// pub struct Chicken{
//     view: StillImage,
//     health: i32,
//     health_bar: StillImage,
//     player: bool,               //p1 = true, p2 = false
// }
// impl Chicken {
//     pub async fn new(x:f32, y:f32, player: bool) -> Self {
//         if player {
//             Chicken {
//                 view: StillImage::new("assets/class_images/p1/idle.png", 50.0, 100.0, x, y, true, 1.0).await,
//                 health: 10,
//                 health_bar: StillImage::new("assets/health/p1/health10.png")
//             }
//         } else {
//             Chicken {

//             }
//         }
//     }
// }