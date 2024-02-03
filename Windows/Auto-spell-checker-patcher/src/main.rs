use ascu::Downloader;
use std::env::args;
use std::process::Command;
use sysinfo::{System, SystemExt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        return;
    };

    let path: &str = &(args[1]);
    let pid = args[2].parse::<u32>().unwrap();

    // Kill EXECUTOR_EXE if it's running
    let mut system = System::new_all();
    system.refresh_all();

    for (p, process) in system.get_processes() {
        if p == pid {
            let _ = process.kill(sysinfo::Signal::Kill);
            break;
        }
    }

    let downloader = Downloader::new();
    let result = downloader.download_executor(path).await;

    match result {
        Ok(exe_path) => {
            let _ = Command::new(exe_path)
                .spawn()
                .expect("Failed to execute command");
        }

        Err(error) => {
            println!("{:?}", error);
        }
    }
}
