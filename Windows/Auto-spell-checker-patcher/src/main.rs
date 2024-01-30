use aws::get_lastest;
use dotenv::dotenv;
use std::env;
use std::process::Command;

mod aws;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    };

    let path: &str = &(args[1]);

    let mut object_key = String::new();
    let mut bucket = String::new();
    let client = aws::init_aws(&mut bucket, &mut object_key).await;
    let result = get_lastest(path, &client, &bucket, &object_key).await;

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
