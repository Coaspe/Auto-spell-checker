use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::ProvideCredentials;
use aws_sdk_s3::Client;
use std::io::prelude::*;
use std::{
    env::var,
    fs::{remove_file, File},
};

pub async fn init_aws(bucket: &mut String, object_key: &mut String) -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    config
        .credentials_provider()
        .expect("No AWS credentials provider was configured")
        .provide_credentials()
        .await
        .expect("No AWS credentials were provided");

    let client = Client::new(&config);

    bucket.push_str(var("BUCKET").unwrap().as_str());
    object_key.push_str(var("OBJECT_KEY").unwrap().as_str());

    client
}
pub async fn get_lastest(
    path: &str,
    client: &Client,
    bucket: &String,
    object_key: &String,
) -> Result<usize, Box<dyn std::error::Error>> {
    let _ = remove_file(path);

    let mut file = File::create(path)?;
    let mut object = client
        .get_object()
        .bucket(bucket)
        .key(object_key)
        .send()
        .await?;

    let mut bytes_len = 0;
    while let Some(bytes) = object.body.try_next().await? {
        bytes_len += bytes.len();
        file.write_all(&bytes)?;
    }

    Ok(bytes_len)
}
