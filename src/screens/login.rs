// Created by Cameron
// April 28th, 2026
// This file contains the code for the login screen

use macroquad::prelude::*;
use serde::Deserialize;
use crate::utils::database::{create_database_client, DatabaseTable};
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::ui::label::Label;

#[derive(Debug, Deserialize)]
pub struct LoginRecord {
    username: String,
    password: String,
    rank: String,
    money: String
}
impl LoginRecord {
    fn new(username: String, password: String, rank: String, money: String) -> Self {
        LoginRecord { username: username, password: password, rank: rank, money: money }
    }
 }

pub async fn run() -> String{
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
    let usernamelabel = Label::new("Username:", 50.0, 30.0, 24);
    let passwordlabel = Label::new("Password:", 50.0, 130.0, 24);
    let mut lbl_message = Label::new("", 50.0, 220.0, 24);

    let create_table_sql = "CREATE TABLE IF NOT EXISTS poultrywars (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, password TEXT NOT NULL, rank TEXT NOT NULL, money TEXT NOT NULL)";
    if let Err(err) = client.execute_sql(create_table_sql).await {
        lbl_message.set_text(format!("Failed to ensure user table exists: {}", err));
        println!("Failed to ensure user table exists: {}", err);
    }

    loop {
        clear_background(WHITE);

        txt_username.draw();
        txt_password.draw();
        usernamelabel.draw();
        passwordlabel.draw();
        lbl_message.draw();

        // Check for button click
        if create_button.click() {
            let username = txt_username.get_text();
            let password = txt_password.get_text();

            if username.is_empty() || password.is_empty() {
                lbl_message.set_text("Please enter both a username and password.".to_string());
                println!("Please enter both a username and password.");
            } else {
                let new_record = DatabaseTable {
                    id: 0,
                    username,
                    password,
                    rank: "Embryo".to_string(),
                    money: "0".to_string(),
                };

                match client.insert_record("poultrywars", &new_record).await {
                    Ok(id) => {
                        lbl_message.set_text(format!("Account created (ID: {})", id));
                        println!("Record inserted with ID: {}", id);
                    }
                    Err(err) => {
                        lbl_message.set_text(format!("Failed to insert record: {}", err));
                        println!("Failed to insert record: {}", err);
                    }
                }
            }
        }

        if login_button.click() {
            let username = txt_username.get_text();
            let password = txt_password.get_text();

            if username.is_empty() || password.is_empty() {
                lbl_message.set_text("Please enter both a username and password.".to_string());
                println!("Please enter both a username and password.");
            } else {
            //     let sql = "SELECT username, password FROM poultrywars ORDER BY id";
            //     match client.fetch_with_sql::<LoginRecord>(sql).await {
            //         Ok(records) => {
            //             let login_successful = records.iter().any(|record| record.username == username && record.password == password);
            //             if login_successful {
            //                 lbl_message.set_text("Login successful!".to_string());
            //                 println!("Login successful!");
            //             } else {
            //                 lbl_message.set_text("Invalid username or password.".to_string());
            //                 println!("Invalid username or password.");
            //             }
            //         }
            //         Err(err) => {
            //             lbl_message.set_text(format!("Failed to load login records: {}", err));
            //             println!("Failed to load login records: {}", err);
            //         }
            //     }
                let mut loginfo = 
                //result<vec<database>>
                let records_err = client.fetch_table("poultry_wars").await;
                if let Ok(records) = records_err {
                    for player in &records {
                        if player.username == username && player.password == password {

                        }
                    } else {
                        println!("MAKE A LBL TO SHOW THIS GENIUS. USER OR PASSWORD DNE")
                    }
                } else if let Err(e) = records_err {
                    println!("FATAL: FETCH ERR: {}", e)
                }
            }
        }
        fn credentials(){
            
        }
        next_frame().await
    }
}

