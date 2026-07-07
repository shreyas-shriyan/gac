use serde::{Deserialize, Serialize};
use std::{env, fs};

// Define structs that match the TOML structure
#[derive(Deserialize, Serialize)]
struct AppConfig {
    profile: Vec<Config>,
}

#[derive(Deserialize, Serialize)]
struct Config {
    nickname: String,
    username: String,
    email: String,
}

fn get_config_dir_path() -> std::path::PathBuf {
    #[cfg(target_os = "linux")]
    let config_dir = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", env::var("HOME").unwrap()));
    let mut path = std::path::PathBuf::from(config_dir);
    let app_name = env::var("CARGO_PKG_NAME").unwrap();
    path.push(app_name);

    return path;
}

fn get_config_path() -> std::path::PathBuf {
    let mut path = get_config_dir_path();
    path.push("config.toml");

    return path;
}

// List Functions
fn list_profiles() {
    let toml_string = fs::read_to_string(get_config_path()).expect("Failed to read config.toml");
    let app_config: AppConfig = toml::from_str(&toml_string).expect("Failed to parse TOML");

    for profile in app_config.profile {
        println!(
            "Nickname: {}, Username: {}",
            profile.nickname, profile.username
        );
    }
}

// Save Functions
fn save_config(app_config: &AppConfig) {
    // Convert the struct back to a formatted TOML string
    let toml_string = toml::to_string(app_config).expect("Failed to serialize to TOML");

    let mut path = get_config_dir_path();

    fs::create_dir_all(&path).expect("Failed to create config directory");

    path.push("config.toml");

    // Write it to the file
    fs::write(path, toml_string).expect("Failed to write to file");
}

fn add_profile(nickname: String, username: String, email: String) {
    let mut app_config = if get_config_path().exists() {
        let toml_string =
            fs::read_to_string(get_config_path()).expect("Failed to read config.toml");
        toml::from_str(&toml_string).expect("Failed to parse TOML")
    } else {
        AppConfig {
            profile: Vec::new(),
        }
    };

    let new_profile = Config {
        nickname,
        username,
        email,
    };

    // check if the nickname or email already exists in the profiles
    for profile in &app_config.profile {
        if profile.nickname == new_profile.nickname {
            println!("Nickname '{}' already exists.", new_profile.nickname);
            return;
        }
        if profile.email == new_profile.email {
            println!("Email '{}' already exists.", new_profile.email);
            return;
        }
    }

    app_config.profile.push(new_profile);
    save_config(&app_config);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No CLI arguments provided.");
        println!("Choose a command: list , add , remove");
        return;
    }

    let command = args[1].as_str();

    match command {
        "list" => list_profiles(),
        "add" => {
            if args.len() < 5 {
                println!("Usage: add <nickname> <username> <email>");
                return;
            }
            let nickname = args[2].clone();
            let username = args[3].clone();
            let email = args[4].clone();
            add_profile(nickname, username, email);
        }
        _ => println!("Unknown command: {}", command),
    }
}
