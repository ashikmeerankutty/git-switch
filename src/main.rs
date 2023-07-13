use std::env;
use std::path::Path;
use std::fs::File;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde()]
struct GitConfig {
    host: String,
    username: String,
    email: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: git-switch <config_file>");
        return;
    }
    let config_file_path = Path::new(&args[1]);
    let config_file = File::open(config_file_path).expect("file not found");

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
