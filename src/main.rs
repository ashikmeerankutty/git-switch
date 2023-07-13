use std::fs::File;
use std::process::Command;
use serde::Deserialize;
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

    let config_file = File::open(config_file_path).expect("Config not found, Create a .git-switch.json file in your home directory.");

    let configs: Vec<GitConfig> = serde_json
        ::from_reader(config_file)
        .expect("Failed to read the config file");

    for config in configs {
        let output = Command::new("git")
            .arg("remote")
            .arg("get-url")
            .arg("origin")
            .output()
            .expect("Failed to execute 'git remote get-url origin'");

        let origin_url = String::from_utf8_lossy(&output.stdout);

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
