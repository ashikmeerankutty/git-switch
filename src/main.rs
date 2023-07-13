use serde::Deserialize;
use std::fs::File;
use std::process::Command;
extern crate dirs;

#[derive(Debug, Deserialize)]
#[serde()]
struct GitConfig {
    host: String,
    username: String,
    email: String,
}

fn main() {
    let home_dir = dirs::home_dir().expect("Failed to retrieve the home directory");

    let config_file_path = home_dir.join(".git-switch.json");

    let config_file = File::open(config_file_path)
        .expect("Config not found, Create a .git-switch.json file in your home directory.");

    let configs: Vec<GitConfig> =
        serde_json::from_reader(config_file).expect("Failed to read the config file");

    for config in configs {
        let output: std::process::Output = Command::new("git")
            .arg("remote")
            .arg("get-url")
            .arg("origin")
            .output()
            .expect("Failed to execute 'git remote get-url origin'");

        let origin_url: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

        let current_user_name_output: std::process::Output = Command::new("git")
            .arg("config")
            .arg("user.name")
            .output()
            .expect("Failed to execute 'git config user.name'");

        let current_user_email_output: std::process::Output = Command::new("git")
            .arg("config")
            .arg("user.email")
            .output()
            .expect("Failed to execute 'git config user.email'");

        let current_user_name: std::borrow::Cow<'_, str> =
            String::from_utf8_lossy(&current_user_name_output.stdout);

        let current_user_email: std::borrow::Cow<'_, str> =
            String::from_utf8_lossy(&current_user_email_output.stdout);

        if current_user_name.trim() != config.username || current_user_email.trim() != config.email {
            if origin_url.contains(&config.host) {
                let username_output = Command::new("git")
                    .arg("config")
                    .arg("--global")
                    .arg("user.name")
                    .arg(&config.username)
                    .output()
                    .expect("Failed to execute 'git config --global user.name'");
                let email_output = Command::new("git")
                    .arg("config")
                    .arg("--global")
                    .arg("user.email")
                    .arg(&config.email)
                    .output()
                    .expect("Failed to execute 'git config --global user.email'");

                if username_output.status.success() && email_output.status.success() {
                    println!("Successfully switched Git configuration to: {:?}", config);
                } else {
                    println!("Failed to switch Git configuration");
                }
            }
        }
    }
}
