use aws::get_lastest;
use dotenv::dotenv;
use std::env;
use std::process::Command;

mod aws;

const EXECUTOR_URL: &str = "EXECUTOR_URL";
#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    };

    let path: &str = &(args[1]);
    let url = env::var(EXECUTOR_URL).unwrap();
    let result = get_lastest(path, &url).await;

    match result {
        Ok(_) => {
            let _ = Command::new(path)
                .spawn()
                .expect("Failed to execute command");
        }

        Err(error) => {
            println!("{:?}", error);
        }
    }
}
