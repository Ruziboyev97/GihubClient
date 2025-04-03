// src/config.rs
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use crate::ui::get_input;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: String,
}

pub struct TokenManager {
    config_path: String,
}

impl TokenManager {
    pub fn new() -> Self {
        TokenManager {
            config_path: "config.json".to_string(),
        }
    }

    // Получение токена или запрос нового
    pub fn get_or_prompt_token(&self) -> io::Result<String> {
        match self.load_token() {
            Some(token) => Ok(token),
            None => {
                let input_token = get_input("Введите ваш GitHub токен: ");
                self.save_token(&input_token)?;
                Ok(input_token)
            }
        }
    }

    // Обновление токена
    pub fn update_token(&self) -> io::Result<String> {
        let input_token = get_input("Введите новый GitHub токен: ");
        self.save_token(&input_token)?;
        Ok(input_token)
    }

    // Сохранение токена в файл
    fn save_token(&self, token: &str) -> io::Result<()> {
        let config = Config {
            token: token.to_string(),
        };
        let config_json = serde_json::to_string(&config)?;
        fs::write(&self.config_path, config_json)?;
        println!("Токен успешно сохранен!");
        Ok(())
    }

    // Загрузка токена из файла
    fn load_token(&self) -> Option<String> {
        match fs::read_to_string(&self.config_path) {
            Ok(contents) => {
                match serde_json::from_str::<Config>(&contents) {
                    Ok(config) => Some(config.token),
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }
}