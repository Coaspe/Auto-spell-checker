use ascu::Downloader;
use std::env::args;
use std::process::Command;
use sysinfo::{Pid, System};

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        return;
    };

    let path: &str = &(args[1]);
    let pid = args[2].parse::<usize>().unwrap();
    println!("path: {}", path);
    println!("pid: {}", pid);
    // Kill EXECUTOR_EXE if it's running
    let system = System::new_all();
    // system.refresh_processes();

    if let Some(process) = system.process(Pid::from(pid)) {
        println!("Killing process: {:?}", process.name());
        let tt = process.kill();
        println!("Killed: {:?}", tt);
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
            loop {}
        }
    }
}
