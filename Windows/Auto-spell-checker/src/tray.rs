use crate::util;
use std::process::Child;
use std::{io, process::Command};
use std::sync::mpsc::sync_channel;
use tray_item::{IconSource, TrayItem};
use util::CURRENT_VERSION;

const GUIDANCE: &str =
    "&& echo --------------------------------------------------------------------- \
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

/// Represents the different types of messages that can be sent to the tray.
enum Message {
    /// Indicates a request to quit the application.
    Quit,
    /// Indicates a request to show the window.
    ShowWindow,
    /// Indicates a request to report something.
    Report,
}

/// Creates guidance by executing a command with the specified program and arguments.
fn create_guidance() -> Result<Option<Child>, io::Error> {
    let mut cmd = Command::new(COMMAND_PROGRAM);
    cmd.args(&[
        "/C",
        &("echo ".to_string() + CURRENT_VERSION + " ver " + GUIDANCE),
    ]);

    Ok(Some(cmd.spawn()?))
}

fn create_console(console: &mut Option<Child>) {
    // 현재 콘솔이 None이 아니라면 종료
    if let Some(mut current_child) = console.take() {
        // 현재 콘솔 프로세스를 종료
        let _ = current_child.kill();
    }

    // 새로운 가이던스 생성 시도
    if let Ok(new_child) = create_guidance() {
        // 새로운 가이던스로 콘솔 업데이트
        *console = new_child;
    }
}

/// Initializes the system tray with menu items and message handling.
pub fn init_tray() {
    // Create a new tray item with the specified title and icon source
    let mut tray = TrayItem::new(TRAY_TITLE, IconSource::Resource(APP_ICON)).unwrap();
    let mut console: Option<Child> = None;

    // Add a label to the tray item
    tray.add_label(TRAY_TITLE).unwrap();

    // Create a synchronous channel for communication between threads
    let (tx, rx) = sync_channel(1);

    // Create guidance for the application
    create_console(&mut console);

    // Clone the sender for showing the window and add a menu item with a closure
    let show_window_tx = tx.clone();
    tray.add_menu_item(USAGE, move || {
        show_window_tx.send(Message::ShowWindow).unwrap();
    })
    .unwrap();

    // Clone the sender for reporting and add a menu item with a closure
    let report_tx = tx.clone();
    tray.add_menu_item(REPORT, move || {
        report_tx.send(Message::Report).unwrap();
    })
    .unwrap();

    // Add a separator to the tray menu
    tray.inner_mut().add_separator().unwrap();

    // Clone the sender for quitting and add a menu item with a closure
    let quit_tx = tx.clone();
    tray.add_menu_item(QUIT, move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    // Enter the message handling loop
    loop {
        match rx.recv() {
            // If the Quit message is received, exit the application
            Ok(Message::Quit) => {
                std::process::exit(0);
            }

            // If the ShowWindow message is received, create guidance
            Ok(Message::ShowWindow) => {
                create_console(&mut console);
            }

            // If the Report message is received, open the report URL
            Ok(Message::Report) => {
                let _ = open::that(REPORT_URL);
            }
            _ => {}
        }
    }
}
