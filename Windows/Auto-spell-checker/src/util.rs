use ascu::{Downloader, PATCHER_EXE};
use notify_rust::Notification;
use regex::Regex;
use reqwest::Error;
use std::{env, path::Path, process::Command};
use sysinfo::System;

const CHECK_VERSION_ERR_MSG: &str = "버전을 확인하는데 실패했습니다.";
const CHECK_VERSION_MSG: &str = "최신 버젼이 존재합니다 업데이트를 진행합니다.";
const APP_EXE_NAME: &str = "auto_spell_checker.exe";
const Q: &str = "q";
const URL: &str = "https://m.search.naver.com/p/csearch/ocontent/util/SpellerProxy";
const USER_AGENT: &str = "user-agent";
const PASSPORT_KEY: &str = "passportKey";
const USER_AGENT_VAL: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
AppleWebKit/537.36 (KHTML, like Gecko) \
Chrome/57.0.2987.133 Safari/537.36";
const REFERER: &str = "referer";
const REFERER_VAL: &str = "https://search.naver.com/";
const MESSAGE: &str = "message";
const RESULT: &str = "result";
const NOTAG_HTML: &str = "notag_html";
const APP_NAME: &str = "자동 맞춤법 검사기";
const BASE_URL: &str = "https://search.naver.com/search.naver?ie=UTF-8&sm=whl_hty&query=%EB%A7%9E%EC%B6%A4%EB%B2%95%EA%B2%80%EC%82%AC%EA%B8%B0";
const COLOR_BLINDNESS: &str = "color_blindness";
const COLOR_BLINDNESS_VAL: &str = "0";
pub const CURRENT_VERSION: &str = "0.1.0";
const SUMMARY_S_CHECK_VERSION: &str = "최신 버젼 확인";
const SUMMARY_F_CHECK_VERSION: &str = "버전을 확인 실패";
const USERS_PATH: &str = "C:\\Users";
const USERNAME: &str = "USERNAME";
const PARENT_FOLRDER: &str = "Auto spell checker";
/// Checks if the current process already exists and terminates it if there is more than one instance.
pub fn does_exist() {
    let s = System::new_all();
    let id = std::process::id();
    for p in s.processes_by_exact_name(APP_EXE_NAME) {
        if p.pid().as_u32() != id {
            p.kill();
        }
    }
}

/// Displays a notification with the given message and summary.
///
/// # Arguments
///
/// * `msg` - The message to be displayed in the notification.
/// * `summary` - The summary of the notification.
///
/// # Example
///
/// ```
/// notify("Hello, world!", "Notification");
/// ```
pub fn notify(msg: &str, summary: &str) {
    let _ = Notification::new()
        .appname(APP_NAME)
        .summary(summary)
        .body(msg)
        .auto_icon()
        .show();
}

/// Retrieves the passport key from the server using the provided `client`.
///
/// # Arguments
///
/// * `client` - A reference to a `reqwest::Client` instance.
///
/// # Returns
///
/// * `Result<String, Error>` - A `Result` containing the passport key as a `String` if successful, or an `Error` if an error occurred.
///
/// # Example
///
/// ```rust
/// use reqwest::Client;
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let client = Client::new();
///     let passport_key = get_passport_key(&client).await?;
///     println!("Passport Key: {}", passport_key);
///     Ok(())
/// }
/// ```
pub async fn get_passport_key(client: &reqwest::Client) -> Result<String, Error> {
    let request = client
        .get(BASE_URL)
        .header(USER_AGENT, USER_AGENT_VAL)
        .send()
        .await;

    return match request {
        Ok(res) => {
            let body = res.text().await?;
            let re = Regex::new(r#"(?i)passportKey=([^"'\s]+)"#).unwrap();

            if let Some(captures) = re.captures(&body) {
                if let Some(value) = captures.get(1) {
                    return Ok(value.as_str().to_string());
                }
            }

            Ok(String::from(""))
        }

        Err(e) => Err(e),
    };
}

/// Retrieves a formatted string by sending a POST request to a specified URL with the given parameters.
///
/// # Arguments
///
/// * `text` - The text to be sent in the request.
/// * `client` - The reqwest client used to send the request.
/// * `passport_key` - The passport key used as a parameter in the request.
///
/// # Returns
///
/// Returns a Result containing the formatted string if the request is successful, or an error if the request fails.
///
/// # Errors
///
/// This function can return any error that implements the `std::error::Error` trait.
///
/// # Example
///
/// ```rust
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new();
///     let passport_key = "your_passport_key";
///     let text = "Hello, world!";
///
///     match get_formatted_string(text, &client, passport_key).await {
///         Ok(formatted_string) => println!("Formatted string: {}", formatted_string),
///         Err(err) => eprintln!("Error: {}", err),
///     }
/// }
/// ```
pub async fn get_formatted_string(
    text: &str,
    client: &reqwest::Client,
    passport_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let params = [
        (PASSPORT_KEY, passport_key),
        (COLOR_BLINDNESS, COLOR_BLINDNESS_VAL),
        (Q, text),
    ];

    let response = client
        .post(URL)
        .header(USER_AGENT, USER_AGENT_VAL)
        .header(REFERER, REFERER_VAL)
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = serde_json::from_str(&(response.text().await?))?;
        let no_tag = json[MESSAGE][RESULT][NOTAG_HTML]
            .as_str()
            .unwrap()
            .to_string();
        Ok(no_tag)
    } else {
        let error = response.error_for_status().unwrap_err();
        Err(From::from(error))
    }
}

pub async fn manage_version() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = Downloader::new();

    match downloader.check_version(CURRENT_VERSION).await {
        Ok(is_latest) => {
            if !is_latest {
                notify(CHECK_VERSION_MSG, SUMMARY_S_CHECK_VERSION);

                let env_user_name = env::var(USERNAME).unwrap();
                let display_user_name = env_user_name.split(".").collect::<Vec<&str>>().join(" ");
                let patcher_path_without_file_name =
                    String::from(USERS_PATH) + "\\" + &display_user_name + "\\" + PARENT_FOLRDER;

                let patcher_dir_path = Path::new(&patcher_path_without_file_name);
                if !patcher_dir_path.exists() {
                    let _ = std::fs::create_dir_all(patcher_dir_path)?;
                }

                let patcher_path = patcher_dir_path.join(PATCHER_EXE);
                if !patcher_path.exists() {
                    let _ = downloader
                        .download_patcher(patcher_dir_path.to_str().unwrap())
                        .await
                        .unwrap();
                }

                let parent_folder = env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .display()
                    .to_string();

                let _ = Command::new(patcher_path.to_str().unwrap())
                    .arg(parent_folder)
                    .arg(std::process::id().to_string())
                    .spawn()
                    .expect("Failed to execute command");

                std::process::exit(0);
            }
            Ok(())
        }

        Err(err) => {
            notify(CHECK_VERSION_ERR_MSG, SUMMARY_F_CHECK_VERSION);
            Err(From::from(err))
        }
    }
}
