use std::fs::File;
use std::io::prelude::*;
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use dotenv::dotenv;
use std::env;

const EXE_FILE_URL: &str = "https://your-s3-bucket.s3.amazonaws.com/your-exe-file.exe";

pub async  fn download_exe_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Configure AWS S3 client
    let client = S3Client::new(Region::default());

    // Specify bucket and object key
    let bucket_name = "your_bucket_name";
    let object_key = "your_object_key";

    // Prepare request to get the object
    let request = GetObjectRequest {
        bucket: bucket_name.to_owned(),
        key: object_key.to_owned(),
        ..Default::default()
    };

    // Get the object from S3
    let result = client.get_object(request).await?;

    // Extract the body from the response
    let body = result.body.unwrap();
    let mut body_bytes = Vec::new();

    // Read the body into a byte vector
    body.into_async_read().read_to_end(&mut body_bytes).await?;

    // Specify the path to save the downloaded file
    let file_path = "path_to_save_exe_file/your_exe_file_name.exe";

    // Write the byte vector to a file
    let mut file = File::create(file_path)?;
    file.write_all(&body_bytes)?;

    Ok(())
}
