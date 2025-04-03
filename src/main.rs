// src/main.rs
mod config;
mod repository;
mod ui;

use crate::config::TokenManager;
use crate::repository::RepositoryManager;
use crate::ui::{Menu, get_input};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut token_manager = TokenManager::new();
    let mut token = token_manager.get_or_prompt_token()?;
    
    let repo_manager = RepositoryManager::new();
    let menu = Menu::new();

    loop {
        menu.display();
        let choice = get_input("Выберите действие (1-6): ");
        
        match choice.as_str() {
            "1" => repo_manager.list_repositories(&token).await?,
            "2" => repo_manager.create_repository(&token).await?,
            "3" => repo_manager.update_repository(&token).await?,
            "4" => repo_manager.delete_repository(&token).await?,
            "5" => {
                token = token_manager.update_token()?;
            },
            "6" => {
                println!("Выход из программы.");
                break;
            },
            _ => println!("Неверный выбор. Пожалуйста, попробуйте снова."),
        }
    }

    Ok(())
}