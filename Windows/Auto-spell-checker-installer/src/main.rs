use aws::get_executor_and_patcher;
use dotenv::dotenv;
use file_system::get_desktop_path;
use std::env;
use std::{env::current_dir, process::Command};
mod aws;
mod file_system;

const USERNAME: &str = "USERNAME";
const USERS_PATH:&str = "C:\\Users";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let desktop_path = get_desktop_path();
    let mut executor_install_path =
        String::from(current_dir().unwrap().to_str().unwrap().to_string());

    let env_user_name = env::var(USERNAME).unwrap();
    let display_user_name = env_user_name.split(".").collect::<Vec<&str>>().join(" ");

    let patcher_install_path = USERS_PATH.to_string() + &display_user_name;

    match desktop_path {
        Some(path) => {
            executor_install_path = path.to_os_string().into_string().unwrap();
        }

        None => {}
    }

    match get_executor_and_patcher(&executor_install_path, &patcher_install_path).await {
        // Installation completed
        Ok(executor_path) => {
            // Execute
            Command::new(executor_path)
                .spawn()
                .expect("Failed to execute process");
        }

        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
