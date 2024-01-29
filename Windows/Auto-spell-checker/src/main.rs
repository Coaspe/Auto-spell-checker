#![windows_subsystem = "windows"]
mod tray;
mod util;
use clipboard_win::{formats::Unicode, get_clipboard_string, set_clipboard};
use futures::executor::block_on;
use inputbot::KeybdKey::{LAltKey, LControlKey};
use util::{does_exist, get_formatted_string, get_passport_key, notify};

const UNKNOWN_MSG: &str = "뭔가 잘못됐습니다. 이우람에게 연락하세요.";
const WRONG_CLIPBOARD_TEXT_MSG: &str = "클립보드에 문자가 아닌 요소가 복사되어 있습니다.";
const PASSPORT_KEY_MSG: &str = "패스포트 키를 가져오는데 실패했습니다. 이우람에게 연락하세요.";
const SUMMARY_F_START: &str = "프로그램 시작을 실패했습니다.";
const SUMMARY_S: &str = "결과가 클립보드에 복사되었습니다.";
const SUMMARY_F: &str = "맞춤법 검사에 실패했습니다.";
const SUMMARY_F_MAX: &str = "300자 이상의 텍스트는 검사할 수 없습니다.";

#[tokio::main]
async fn main() {
    does_exist();

    let client = reqwest::Client::new();

    let result = get_passport_key(&client).await;

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

    std::thread::spawn(|| {
        tray::init_tray();
    });

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
