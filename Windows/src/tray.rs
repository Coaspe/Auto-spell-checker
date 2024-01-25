use std::process::Command;
use std::sync::mpsc::sync_channel;
use tray_item::{IconSource, TrayItem};

const GUIDANCE: &str =
    "echo --------------------------------------------------------------------- \
&& echo 1. 원하는 텍스트를 클립 보드에 복사 (Ctrl + C) \
&& echo 2. Left Ctrl + Left Alt를 순서대로 누르면 자동 맞춤법 검사가 진행됩니다. \
&& echo 3. 자동 맞춤법 검사가 완료되면 클립보드에 자동으로 복사됩니다. \
&& echo 4. 원하는 곳에 붙여넣기 하세요. \
&& echo --------------------------------------------------------------------- \
&& echo 해당 앱은 백그라운드로 실행됩니다. \
&& echo 앱을 종료하거나 사용법을 다시 보고 싶다면 숨겨진 아이콘에서 우클릭하세요. \
&& echo 문의 사항은 이우람에게 해주세요. && pause";
const REPORT_URL: &str = "https://auto-spell-checker.web.app/";
const USAGE: &str = "사용법";
const REPORT: &str = "홈페이지";
const QUIT: &str = "종료";
const TRAY_TITLE: &str = "Auto Spell Checker";
const COMMAND_PROGRAM: &str = "cmd";
const APP_ICON: &str = "app-icon";

enum Message {
    Quit,
    ShowWindow,
    Report,
}

fn create_guidance() {
    let mut cmd = Command::new(COMMAND_PROGRAM);
    cmd.args(&["/C", GUIDANCE]);
    let _ = cmd.spawn().unwrap();
}

pub fn init_tray() {
    let mut tray = TrayItem::new(TRAY_TITLE, IconSource::Resource(APP_ICON)).unwrap();

    tray.add_label(TRAY_TITLE).unwrap();

    let (tx, rx) = sync_channel(1);

    create_guidance();

    let show_window_tx = tx.clone();
    tray.add_menu_item(USAGE, move || {
        show_window_tx.send(Message::ShowWindow).unwrap();
    })
    .unwrap();

    let report_tx = tx.clone();
    tray.add_menu_item(REPORT, move || {
        report_tx.send(Message::Report).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item(QUIT, move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                std::process::exit(0);
            }

            Ok(Message::ShowWindow) => {
                create_guidance();
            }

            Ok(Message::Report) => {
                let _ = open::that(REPORT_URL);
            }
            _ => {}
        }
    }
}
