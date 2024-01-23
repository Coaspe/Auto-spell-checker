#![windows_subsystem = "windows"]
mod tray;
use clipboard_win::{formats, get_clipboard_string, set_clipboard};
use futures::executor::block_on;
use inputbot::KeybdKey::{LAltKey, LControlKey};
use notify_rust::Notification;

const ERROR_MSG: &str = "뭔가 잘못됐습니다. 이우람에게 연락하세요.";
const SUMMARY_S: &str = "결과가 클립보드에 복사되었습니다.";
const SUMMARY_F: &str = "맞춤법 검사에 실패했습니다.";
const SUMMARY_F_MAX: &str = "300자 이상의 텍스트는 검사할 수 없습니다.";
const PASSPORTKEY: &str = "passportKey";
const PASSPROTKEYVAL: &str = "73243ceca1b32f8d209eef0ec3e29034fae85ae9";
const COLORBLINDNESS: &str = "color_blindness";
const COLORBLINDNESSVAL: &str = "0";
const Q: &str = "q";
const URL: &str = "https://m.search.naver.com/p/csearch/ocontent/util/SpellerProxy";
const USERAGENT: &str = "user-agent";
const USERAGENTVAL: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                            AppleWebKit/537.36 (KHTML, like Gecko) \
                            Chrome/57.0.2987.133 Safari/537.36";
const REFERER: &str = "referer";
const REFERERVAL: &str = "https://search.naver.com/";
const MESSAGE: &str = "message";
const RESULT: &str = "result";
const NOTAG_HTML: &str = "notag_html";
const APP_NAME: &str = "자동 맞춤법 검사기";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    std::thread::spawn(move || {
        tray::init_tray();
    });

    LAltKey.bind(move || {
        if LControlKey.is_pressed() {
            if let Ok(text) = get_clipboard_string() {
                if text.len() > 300 {
                    notify(SUMMARY_F, SUMMARY_F_MAX);
                    return;
                }

                let tt = text;
                let _ = &rt.block_on(async {
                    if let Ok(formatted_string) = block_on(get_formatted_string(&tt, &client)) {
                        if let suceess_msg = formatted_string.clone().as_str() {
                            let _clip = set_clipboard(formats::Unicode, formatted_string);
                            notify(suceess_msg, SUMMARY_S);
                        }
                    } else {
                        notify(ERROR_MSG, SUMMARY_F);
                    }
                });
            } else {
                notify(ERROR_MSG, SUMMARY_S);
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

async fn get_formatted_string(
    text: &str,
    client: &reqwest::Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let params = [
        (PASSPORTKEY, PASSPROTKEYVAL),
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
        let tt = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&tt)?;
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
