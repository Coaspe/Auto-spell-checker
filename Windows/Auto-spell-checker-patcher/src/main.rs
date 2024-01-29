use utils::download_exe_file;
use std::process::Command;
use std::env;

mod utils;

#[tokio::main]
async fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {return};
   
   let path: &str = &(args[1]);
   let result = download_exe_file(path).await;

    match result {
        Ok(path) => {
        let _ = Command::new(path)
        .spawn()
        .expect("Failed to execute command");
        }

        Err(error) => {
            println!("{:?}", error);
        }
    }
}
