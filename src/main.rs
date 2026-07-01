use serde::{Deserialize, Serialize};
use std::{env, fs};

enum Command {
    list,
}

// Define structs that match the TOML structure
#[derive(Deserialize, Serialize, Debug)]
struct AppConfig {
    profile: Vec<Config>,
}

#[derive(Deserialize, Serialize, Debug)]
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

fn save_config(app_config: &AppConfig) {
    // Convert the struct back to a formatted TOML string
    let toml_string = toml::to_string(app_config).expect("Failed to serialize to TOML");

    let mut path = get_config_dir_path();

    fs::create_dir_all(&path).expect("Failed to create config directory");

    path.push("config.toml");

    // Write it to the file
    fs::write(path, toml_string).expect("Failed to write to file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("First CLI argument: {}", args[1]);
    } else {
        println!("No CLI arguments provided.");
    }

    // let profile = AppConfig {
    //     profile: vec![
    //         Config {
    //             nickname: String::from("personal"),
    //             username: String::from("shreyas-shriyan"),
    //             email: String::from("shreyshriyan@gmail.com"),
    //         },
    //         Config {
    //             nickname: String::from("work"),
    //             username: String::from("shre"),
    //             email: String::from("shreyshriyan@gmail.com"),
    //         },
    //     ],
    // };

    //   save_config(&profile);

    let toml_string = fs::read_to_string(get_config_path()).expect("Failed to read config.toml");

    println!("TOML String: {}", toml_string);
}
