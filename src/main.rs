#![windows_subsystem = "windows"]
mod tray;
use clipboard_win::{formats::Unicode, get_clipboard_string, set_clipboard};
use futures::executor::block_on;
use inputbot::KeybdKey::{LAltKey, LControlKey};
use notify_rust::Notification;
use regex::Regex;
use reqwest::Error;

const UNKNOWN_MSG: &str = "뭔가 잘못됐습니다. 이우람에게 연락하세요.";
const WRONG_CLIPBOARD_TEXT_MSG: &str = "클립보드에 문자가 아닌 요소가 복사되어 있습니다.";
const PASSPORT_KEY_MSG: &str = "패스포트 키를 가져오는데 실패했습니다. 이우람에게 연락하세요.";
const SUMMARY_F_START: &str = "프로그램 시작을 실패했습니다.";
const SUMMARY_S: &str = "결과가 클립보드에 복사되었습니다.";
const SUMMARY_F: &str = "맞춤법 검사에 실패했습니다.";
const SUMMARY_F_MAX: &str = "300자 이상의 텍스트는 검사할 수 없습니다.";
const COLORBLINDNESS: &str = "color_blindness";
const COLORBLINDNESSVAL: &str = "0";
const Q: &str = "q";
const URL: &str = "https://m.search.naver.com/p/csearch/ocontent/util/SpellerProxy";
const USERAGENT: &str = "user-agent";
const PASSPORT_KEY: &str = "passportKey";
const USERAGENTVAL: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
AppleWebKit/537.36 (KHTML, like Gecko) \
Chrome/57.0.2987.133 Safari/537.36";
const REFERER: &str = "referer";
const REFERERVAL: &str = "https://search.naver.com/";
const MESSAGE: &str = "message";
const RESULT: &str = "result";
const NOTAG_HTML: &str = "notag_html";
const APP_NAME: &str = "자동 맞춤법 검사기";
const BASE_URL: &str = "https://search.naver.com/search.naver?ie=UTF-8&sm=whl_hty&query=%EB%A7%9E%EC%B6%A4%EB%B2%95%EA%B2%80%EC%82%AC%EA%B8%B0";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let result = get_passport_key(&client).await;

    // Get passport key
    // passport key is used to get the result of the spelling check
    // it is generated every day you access the naver search page.
    let mut passport_key: String = String::new();

    match result {
        Ok(key) => {
            passport_key = key;
        }
        Err(_) => {
            notify(PASSPORT_KEY_MSG, SUMMARY_F_START);
            return;
        }
    }

    // Set tray icon
    std::thread::spawn(move || {
        tray::init_tray();
    });

    // Key binding
    // if you press ctrl + alt, the text in the clipboard is checked for spelling.
    // if the text is longer than 300 characters, it is not checked.
    let rt = tokio::runtime::Runtime::new().unwrap();

    LAltKey.bind(move || {
        if LControlKey.is_pressed() {
            let clip = get_clipboard_string();
            match clip {
                Ok(text) => {
                    let length = text.len();
                    if length > 300 {
                        notify(SUMMARY_F, SUMMARY_F_MAX);
                        return;
                    }
                    let _ = &rt.block_on(async {
                        if let Ok(formatted_string) =
                            block_on(get_formatted_string(&text, &client, &passport_key))
                        {
                            notify(&formatted_string, SUMMARY_S);
                            let _clip = set_clipboard(Unicode, formatted_string);
                        } else {
                            notify(UNKNOWN_MSG, SUMMARY_F);
                        }
                    });
                }
                Err(e) => match e.raw_code() {
                    126 => {
                        notify(WRONG_CLIPBOARD_TEXT_MSG, SUMMARY_F);
                    }
                    _ => {
                        notify(UNKNOWN_MSG, SUMMARY_F);
                    }
                },
            }
        }
    });

    inputbot::handle_input_events();
}

fn notify(msg: &str, summary: &str) {
    let _ = Notification::new()
        .appname(APP_NAME)
        .summary(summary)
        .body(msg)
        .auto_icon()
        .show();
}

async fn get_passport_key(client: &reqwest::Client) -> Result<String, Error> {
    let request = client
        .get(BASE_URL)
        .header(USERAGENT, USERAGENTVAL)
        .send()
        .await;

    match request {
        Ok(res) => {
            let body = res.text().await.unwrap();
            let re = Regex::new(r#"(?i)passportKey=([^"'\s]+)"#).unwrap();

            if let Some(captures) = re.captures(&body) {
                if let Some(value) = captures.get(1) {
                    return Ok(value.as_str().to_string());
                }
            }
            return Ok(String::from(""));
        }

        Err(e) => {
            return Err(e);
        }
    }
}

async fn get_formatted_string(
    text: &str,
    client: &reqwest::Client,
    passport_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let params = [
        (PASSPORT_KEY, passport_key),
        (COLORBLINDNESS, COLORBLINDNESSVAL),
        (Q, text),
    ];
    let response = client
        .post(URL)
        .header(USERAGENT, USERAGENTVAL)
        .header(REFERER, REFERERVAL)
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = serde_json::from_str(&(response.text().await.unwrap()))?;
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
