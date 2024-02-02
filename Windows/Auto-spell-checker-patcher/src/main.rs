use ascu::Downloader;
use std::env::args;
use std::process::Command;

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        return;
    };

    let downloader = Downloader::new();
    let path: &str = &(args[1]);
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
