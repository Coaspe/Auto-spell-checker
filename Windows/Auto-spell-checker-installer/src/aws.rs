use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::ProvideCredentials;
use aws_sdk_s3::Client;
use std::io::prelude::*;
use std::{
    env::var,
    fs::{create_dir, remove_file, File},
};
pub async fn get_executor_and_patcher(
    executor_install_path: &str,
    pathcer_install_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    config
        .credentials_provider()
        .expect("No AWS credentials provider was configured")
        .provide_credentials()
        .await
        .expect("No AWS credentials were provided");

    let client = Client::new(&config);

    let _ = create_dir(executor_install_path);
    let _ = create_dir(pathcer_install_path);

    let executor_path = executor_install_path.to_string() + "\\Auto spell checker.exe";
    let patcher_path = pathcer_install_path.to_string() + "\\Auto spell checker patcher.exe";

    let _ = remove_file(&executor_path);
    let _ = remove_file(&patcher_path);

    let _ = get_exe(
        &patcher_path,
        &client,
        var("EXECUTOR_BUCKET")?,
        var("EXECUTOR_KEY")?,
    )
    .await?;

    let _ = get_exe(
        &patcher_path,
        &client,
        var("PATCHER_BUCKET")?,
        var("PATCHER_KEY")?,
    )
    .await?;

    Ok(executor_path)
}

async fn get_exe(
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
