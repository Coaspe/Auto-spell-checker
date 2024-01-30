use std::io::Cursor;
use std::{fs::{remove_file, File}};
use std::io::copy;

pub async fn get_lastest(
    path: &str,
    url: &str
) -> Result<u64, Box<dyn std::error::Error>> {
    let _ = remove_file(path);

    let mut file = File::create(path)?;
    let response = reqwest::get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);

    let result = copy(&mut content, &mut file)?;

    Ok(result)
}
