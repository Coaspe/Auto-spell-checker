use std::fs::{create_dir_all, File};
use std::io::{copy, Cursor};
use std::path::Path;

pub const EXECUTOR_EXE: &str = "auto spell checker.exe";
pub const PATCHER_EXE: &str = "auto spell checker patcher.exe";
const PUBLIC_IPV4_DNS: &str = "http://ec2-15-164-94-231.ap-northeast-2.compute.amazonaws.com";
const DOWNLOAD_EXE_URL: &str = "/download_exe";
const LASTEST_VERSION: &str = "/lastest_version";
const OBJECT_KEY: &str = "object_key";
const PATCHER_OBJECT_KEY: &str = "PATCHER_OBJECT_KEY";
const EXECUTOR_OBJECT_KEY: &str = "EXECUTOR_OBJECT_KEY";
pub struct Downloader {}

impl Downloader {
    pub fn new() -> Downloader {
        Self {}
    }

    pub async fn download_executor_and_patcher(
        &self,
        executor_install_path: &str,
        patcher_install_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let executor_path = self
            .download_exe_by_url(
                executor_install_path,
                DOWNLOAD_EXE_URL,
                EXECUTOR_EXE,
                EXECUTOR_OBJECT_KEY,
            )
            .await?;
        let _ = self
            .download_exe_by_url(
                patcher_install_path,
                DOWNLOAD_EXE_URL,
                PATCHER_EXE,
                PATCHER_OBJECT_KEY,
            )
            .await?;
        Ok(executor_path)
    }

    pub async fn download_executor(
        &self,
        install_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .download_exe_by_url(
                install_path,
                DOWNLOAD_EXE_URL,
                EXECUTOR_EXE,
                EXECUTOR_OBJECT_KEY,
            )
            .await?)
    }

    pub async fn download_patcher(
        &self,
        install_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .download_exe_by_url(
                install_path,
                DOWNLOAD_EXE_URL,
                PATCHER_EXE,
                PATCHER_OBJECT_KEY,
            )
            .await?)
    }

    async fn download_exe_by_url(
        &self,
        install_path: &str,
        url: &str,
        exe: &str,
        object_key: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let _ = create_dir_all(install_path)?;
        let path = Path::new(install_path).join(exe);

        let mut params: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        params.insert(OBJECT_KEY, object_key);

        let response = reqwest::Client::new()
            .get(PUBLIC_IPV4_DNS.to_string() + url)
            .query(&params)
            .send()
            .await?;

        let mut file = File::create(&path)?;
        let mut content = Cursor::new(response.bytes().await?);
        let _ = copy(&mut content, &mut file);

        Ok(path.into_os_string().into_string().unwrap())
    }
    pub async fn check_version(&self, current_version: &str) -> Result<bool, reqwest::Error> {
        let lastest_version = reqwest::get(PUBLIC_IPV4_DNS.to_string() + LASTEST_VERSION)
            .await?
            .text()
            .await?;

        let lastest_version = convert_version(&lastest_version);
        let current_version = convert_version(current_version);

        return Ok(current_version >= lastest_version);
    }
}

fn convert_version(s: &str) -> i32 {
    let parts: Vec<&str> = s.split(".").collect();

    let mut result = 0;

    for (index, &part) in parts.iter().enumerate() {
        result += part.parse::<i32>().unwrap_or(0) * 10i32.pow((parts.len() - 1 - index) as u32);
    }

    return result;
}
