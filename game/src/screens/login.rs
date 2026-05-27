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
#[derive(Clone)]
pub struct LoginRecord {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub rank: String,
    pub money: String
}
impl LoginRecord {
    pub fn new(id: i32, username: String, password: String, rank: String, money: String) -> Self {
        LoginRecord { id: id, username: username, password: password, rank: rank, money: money }
    }
 }

pub async fn run() -> Vec<LoginRecord>{
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
    let guest_button: TextButton = TextButton::new(
        450.0,
        250.0,
        100.0,
        50.0,
        "Guest",
        BLUE,
        DARKBLUE,
        24,
    );
    let mut txt_username = TextInput::new(50.0, 50.0, 300.0, 50.0, 24.0);
    let mut txt_password = TextInput::new(50.0, 150.0, 300.0, 50.0, 24.0);
    let usernamelabel = Label::new("Username:", 50.0, 30.0, 24);
    let passwordlabel = Label::new("Password:", 50.0, 130.0, 24);
    let mut lbl_message = Label::new("", 50.0, 220.0, 24);
    let mut lbl_p1 = Label::new("", 450.0, 50.0, 22);
    let mut lbl_p2 = Label::new("", 450.0, 110.0, 22);
    let mut ret_player_data: Vec<LoginRecord> = vec![];

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
        lbl_p1.draw();
        lbl_p2.draw();

        // Check for button click
        if create_button.click() {
            let username = txt_username.get_text();
            let password = txt_password.get_text();

            if username.is_empty() || password.is_empty() {
                lbl_message.set_text("Please enter both a username and password.".to_string());
            } else {
                let new_record = DatabaseTable {
                    id: 0,
                    username,
                    password,
                    rank: "0".to_string(),
                    money: "0".to_string(),
                };

                match client.insert_record("poultrywars", &new_record).await {
                    Ok(id) => {
                        lbl_message.set_text(format!("Account created (ID: {})", id));
                    }
                    Err(err) => {
                        lbl_message.set_text(format!("Failed to insert record: {}", err));
                    }
                }
            }
        }

        if login_button.click() {
            let username = txt_username.get_text();
            let password = txt_password.get_text();

            if username.is_empty() || password.is_empty() {
                lbl_message.set_text("Please enter both a username and password.".to_string());
            } else {
                let mut local_player_info = LoginRecord::new(0, "".to_string(), "".to_string(), "".to_string(), "".to_string());
                let records_err = client.fetch_table("poultrywars").await;
                if let Ok(records) = records_err {
                    for player in &records {
                        if player.username == username && player.password == password {
                            local_player_info.id = player.id.clone();
                            local_player_info.username = player.username.clone();
                            local_player_info.password = player.password.clone();
                            local_player_info.rank = player.rank.clone();
                            local_player_info.money = player.money.clone();
                            break
                        }
                    }
                    if local_player_info.username == "".to_string() {
                        lbl_message.set_text("User or pass do not exist");
                    } else if ret_player_data.len() > 0 {
                        if ret_player_data[0].username == local_player_info.username && ret_player_data[0].password == local_player_info.password {
                            lbl_message.set_text("User already playing");
                        } else {
                            ret_player_data.push(local_player_info);
                        }
                    } else {
                        ret_player_data.push(local_player_info);
                    }
                } else if let Err(e) = records_err {
                    lbl_message.set_text(format!("FATAL: FETCH ERR: {}", e));
                }
            }

            txt_username.set_text("");
            txt_password.set_text("");
        }

        if guest_button.click() || is_key_pressed(KeyCode::K){
            // log in as guest1 on database, rank and money will not be counted for guests, if guest1 is already logged in, log in as guest2
            if ret_player_data.len() == 0 {
                ret_player_data.push(LoginRecord { id: 0, username: "guest1".to_string(), password: "guest".to_string(), rank: "0".to_string(), money: "0".to_string() });
            } else if ret_player_data.len() == 1{
                ret_player_data.push(LoginRecord { id: 0, username: "guest2".to_string(), password: "guessed".to_string(), rank: "0".to_string(), money: "0".to_string() });
            }
        }

        if ret_player_data.len() > 1 {
            return ret_player_data;
        }

        match ret_player_data.get(0) {
            Some(info) => {
                lbl_p1.set_text(format!("P1: {}", info.username));
            }
            None => {}
        }
        match ret_player_data.get(1) {
            Some(info) => {
                lbl_p2.set_text(format!("P2: {}", info.username));
            }
            None => {}
        }

        next_frame().await
    }
}

