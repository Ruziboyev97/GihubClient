// src/ui.rs
use std::io::{self, Write};

pub struct Menu;

impl Menu {
    pub fn new() -> Self {
        Menu
    }

    pub fn display(&self) {
        println!("\n=== GitHub Repository Manager ===");
        println!("1. Показать все репозитории");
        println!("2. Создать новый репозиторий");
        println!("3. Обновить репозиторий");
        println!("4. Удалить репозиторий");
        println!("5. Изменить токен");
        println!("6. Выход");
    }
}

// Функция для получения ввода пользователя
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}