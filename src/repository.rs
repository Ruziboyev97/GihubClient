// src/repository.rs
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use crate::ui::get_input;

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
}

#[derive(Serialize, Debug)]
pub struct CreateRepoRequest {
    name: String,
    description: Option<String>,
    private: bool,
}

pub struct RepositoryManager {
    client: Client,
}

impl RepositoryManager {
    pub fn new() -> Self {
        RepositoryManager {
            client: Client::new(),
        }
    }

    // Получение списка репозиториев
    pub async fn list_repositories(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://api.github.com/user/repos")
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "github-token-app")
            .send()
            .await?;
        
        if response.status().is_success() {
            let repositories: Vec<Repository> = response.json().await?;
            
            if repositories.is_empty() {
                println!("У вас нет репозиториев.");
            } else {
                println!("\nСписок ваших репозиториев:");
                for (i, repo) in repositories.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, repo.name, repo.html_url);
                    if let Some(desc) = &repo.description {
                        if !desc.is_empty() {
                            println!("   Описание: {}", desc);
                        }
                    }
                }
            }
        } else {
            println!("Ошибка: {}", response.status());
        }
        
        Ok(())
    }

    // Создание нового репозитория
    pub async fn create_repository(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let name = get_input("Введите имя для нового репозитория: ");
        let description = get_input("Введите описание (нажмите Enter, чтобы пропустить): ");
        let is_private = get_input("Сделать репозиторий приватным? (y/n): ").to_lowercase() == "y";
        
        let repo_request = CreateRepoRequest {
            name,
            description: if description.is_empty() { None } else { Some(description) },
            private: is_private,
        };
        
        let response = self.client
            .post("https://api.github.com/user/repos")
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "github-token-app")
            .json(&repo_request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let repository: Repository = response.json().await?;
            println!("Репозиторий успешно создан: {}", repository.html_url);
        } else {
            println!("Ошибка при создании репозитория: {}", response.status());
            let error_text = response.text().await?;
            println!("Детали ошибки: {}", error_text);
        }
        
        Ok(())
    }

    // Обновление репозитория
    pub async fn update_repository(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Сначала получаем список репозиториев
        let response = self.client
            .get("https://api.github.com/user/repos")
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "github-token-app")
            .send()
            .await?;
        
        if !response.status().is_success() {
            println!("Ошибка при получении списка репозиториев: {}", response.status());
            return Ok(());
        }
        
        let repositories: Vec<Repository> = response.json().await?;
        
        if repositories.is_empty() {
            println!("У вас нет репозиториев для обновления.");
            return Ok(());
        }
        
        println!("\nВыберите репозиторий для обновления:");
        for (i, repo) in repositories.iter().enumerate() {
            println!("{}. {}", i + 1, repo.name);
        }
        
        let choice = get_input("Введите номер репозитория: ");
        let index = match choice.parse::<usize>() {
            Ok(i) if i > 0 && i <= repositories.len() => i - 1,
            _ => {
                println!("Неверный выбор.");
                return Ok(());
            }
        };
        
        let repo = &repositories[index];
        
        // Получаем новую информацию
        let new_description = get_input(&format!("Введите новое описание для {} (Enter чтобы пропустить): ", repo.name));
        
        // Создаем запрос на обновление
        let mut update_data = serde_json::Map::new();
        if !new_description.is_empty() {
            update_data.insert("description".to_string(), serde_json::Value::String(new_description));
        }
        
        if update_data.is_empty() {
            println!("Нет данных для обновления.");
            return Ok(());
        }
        
        // Отправляем запрос
        let response = self.client
            .patch(&format!("https://api.github.com/repos/{}/{}", "owner", repo.name))
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "github-token-app")
            .json(&update_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            println!("Репозиторий успешно обновлен!");
        } else {
            println!("Ошибка при обновлении репозитория: {}", response.status());
            let error_text = response.text().await?;
            println!("Детали ошибки: {}", error_text);
        }
        
        Ok(())
    }

    // Удаление репозитория
    pub async fn delete_repository(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Получаем список репозиториев
        let response = self.client
            .get("https://api.github.com/user/repos")
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "github-token-app")
            .send()
            .await?;
        
        if !response.status().is_success() {
            println!("Ошибка при получении списка репозиториев: {}", response.status());
            return Ok(());
        }
        
        let repositories: Vec<Repository> = response.json().await?;
        
        if repositories.is_empty() {
            println!("У вас нет репозиториев для удаления.");
            return Ok(());
        }
        
        println!("\nВыберите репозиторий для удаления:");
        for (i, repo) in repositories.iter().enumerate() {
            println!("{}. {}", i + 1, repo.name);
        }
        
        let choice = get_input("Введите номер репозитория (или 'all' для удаления всех): ");
        
        if choice.to_lowercase() == "all" {
            let confirm = get_input("Вы уверены, что хотите удалить ВСЕ репозитории? (y/n): ");
            if confirm.to_lowercase() != "y" {
                println!("Операция отменена.");
                return Ok(());
            }
            
            for repo in &repositories {
                self.delete_single_repository(token, &repo.name).await?;
            }
            
            println!("Все репозитории успешно удалены!");
        } else {
            let index = match choice.parse::<usize>() {
                Ok(i) if i > 0 && i <= repositories.len() => i - 1,
                _ => {
                    println!("Неверный выбор.");
                    return Ok(());
                }
            };
            
            let repo_name = &repositories[index].name;
            let confirm = get_input(&format!("Вы уверены, что хотите удалить репозиторий '{}'? (y/n): ", repo_name));
            
            if confirm.to_lowercase() == "y" {
                self.delete_single_repository(token, repo_name).await?;
                println!("Репозиторий '{}' успешно удален!", repo_name);
            } else {
                println!("Операция отменена.");
            }
        }
        
        Ok(())
    }

    // Вспомогательная функция для удаления одного репозитория
    async fn delete_single_repository(&self, token: &str, repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .delete(&format!("https://api.github.com/repos/{}/{}", "owner", repo_name))
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "github-token-app")
            .send()
            .await?;
        
        if !response.status().is_success() {
            println!("Ошибка при удалении репозитория '{}': {}", repo_name, response.status());
            let error_text = response.text().await?;
            println!("Детали ошибки: {}", error_text);
        }
        
        Ok(())
    }
}