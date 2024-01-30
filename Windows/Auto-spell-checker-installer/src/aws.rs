use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::ProvideCredentials;
use aws_sdk_s3::Client;
use std::io::{prelude::*, Cursor};
use std::{
    env::var,
    fs::{create_dir, remove_file, File},
};
use std::io::copy;

const PATCHER_URL:&str = "PATCHER_URL";
const EXECUTOR_URL: &str = "EXECUTOR_URL";
const EXECUTOR_EXE: &str = "Auto spell checker.exe";
const PATCHER_EXE: &str = "Auto spell checker patcher.exe";
const REGION: &str = "ap-northeast-2";

pub async fn get_executor_and_patcher(
    executor_install_path: &str,
    pathcer_install_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else(REGION);
    let config = aws_config::from_env().region(region_provider).load().await;
    config
        .credentials_provider()
        .expect("No AWS credentials provider was configured")
        .provide_credentials()
        .await
        .expect("No AWS credentials were provided");

    let _ = create_dir(executor_install_path);
    let _ = create_dir(pathcer_install_path);

    let executor_path = executor_install_path.to_string() + "\\" + EXECUTOR_EXE;
    let patcher_path = pathcer_install_path.to_string() + "\\" + PATCHER_EXE;

    let _ = remove_file(&executor_path);
    let _ = remove_file(&patcher_path);

    let _ = download_exe_by_url(
        &patcher_path,
        &var(PATCHER_URL)?
    )
    .await?;

    let _ = download_exe_by_url(
        &executor_path,
        &var(EXECUTOR_URL)?
    )
    .await?;

    Ok(executor_path)
}

pub async fn download_exe_by_url(path: &str, url: &str) -> Result<(), reqwest::Error> {
    let response = reqwest::get(url).await?;
    let mut file = File::create(path).unwrap();
    let mut content = Cursor::new(response.bytes().await?);
    let _ = copy(&mut content, &mut file);
    Ok(())
}

async fn download_exe_by_object_key(
    path: &str,
    client: &Client,
    bucket: String,
    object_key: String,
) -> Result<u8, Box<dyn std::error::Error>> {
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

    Ok(0)
}
