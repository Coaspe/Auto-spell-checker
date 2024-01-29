use std::io::prelude::*;
use std::{env::var, fs::{create_dir, File, remove_file}};
use aws_sdk_s3::config::ProvideCredentials;
use dotenv::dotenv;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

pub async fn download_exe_file(path: &str) -> Result<&str, Box<dyn std::error::Error>> {
    dotenv().ok();

    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    config
        .credentials_provider()
        .expect("No AWS credentials provider was configured")
        .provide_credentials()
        .await
        .expect("No AWS credentials were provided");
    
    let client = Client::new(&config);
    let bucket = var("BUCKET")?;
    let object_key = var("KEY")?;

    // let path = "C:\\Users".to_owned() + "\\" + &whoami::username() + "\\Auto spell checker";
    let mut path_without_file_name: Vec<&str> = path.split("\\").collect();
    let _ = path_without_file_name.pop();

    let _ = create_dir(&path_without_file_name.join("\\"));
    let __ = remove_file(&path);

    let mut file = File::create(&path)?;
    let mut object = client
        .get_object()
        .bucket(bucket)
        .key(object_key)
        .send()
        .await?;

    // let mut byte_count = 0_usize;
    while let Some(bytes) = object.body.try_next().await? {
        // let bytes_len = bytes.len();
        file.write_all(&bytes)?;
        // byte_count += bytes_len;
    }

    Ok(&path)
}
