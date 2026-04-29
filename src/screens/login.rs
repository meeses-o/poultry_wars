// Created by Cameron
// April 28th, 2026
// This file contains the code for the login screen

use macroquad::prelude::*;
use crate::databasing::database::{create_database_client, DatabaseTable};
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;

pub async fn run() {
    let client = create_database_client();
    let create_button: TextButton = TextButton::new(
        250.0,
        250.0,
        150.0,
        50.0,
        "Create Account",
        BLUE,
        DARKBLUE,
        24,
    );
    let login_button: TextButton = TextButton::new(
        50.0,
        250.0,
        100.0,
        50.0,
        "Login",
        BLUE,
        DARKBLUE,
        24,
    );
    let mut txt_username = TextInput::new(50.0, 50.0, 300.0, 50.0, 24.0);
    let mut txt_password = TextInput::new(50.0, 150.0, 300.0, 50.0, 24.0);

    loop {
        clear_background(WHITE);

        txt_username.draw();
        txt_password.draw();

        // Check for button click
        if create_button.click() {
            let username = txt_username.get_text();
            let password = txt_password.get_text();

            if username.is_empty() || password.is_empty() {
                println!("Please enter both a username and password.");
            } else {
                let new_record = DatabaseTable {
                    id: 0,
                    username,
                    password,
                    rank: "Embryo".to_string(),
                    money: "0".to_string(),
                };

                if let Ok(id) = client.insert_record("poultrywars", &new_record).await {
                    println!("Record inserted with ID: {}", id);
                } else {
                    println!("Failed to insert record");
                }
            }
        }

        if login_button.click() {
            let username = txt_username.get_text();
            let password = txt_password.get_text();

            if username.is_empty() || password.is_empty() {
                println!("Please enter both a username and password.");
            } else if let Ok(records) = client.fetch_table("poultrywars").await {
                let login_successful = records.iter().any(|record| record.username == username && record.password == password);

                if login_successful {
                    println!("Login successful!");
                } else {
                    println!("Invalid username or password.");
                }
            } else {
                println!("Failed to load login records.");
            }
        }

        next_frame().await
    }
}

