use std::fs::{create_dir_all, remove_file, File};
use std::io::{copy, Cursor};

const EXECUTOR_EXE: &str = "Auto spell checker.exe";
const PATCHER_EXE: &str = "Auto spell checker patcher.exe";
const BASE_URL: &str = "https://autospellchecker.s3.ap-northeast-2.amazonaws.com/";
const EXECUTOR_URL: &str = "download_executor";
const PATCHER_URL: &str = "download_patcher";
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
            .download_exe_by_url(executor_install_path, EXECUTOR_URL, EXECUTOR_EXE)
            .await?;
        let _ = self
            .download_exe_by_url(patcher_install_path, PATCHER_URL, PATCHER_EXE)
            .await?;

        Ok(executor_path)
    }

    pub async fn download_executor(
        &self,
        install_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .download_exe_by_url(install_path, EXECUTOR_URL, EXECUTOR_EXE)
            .await?)
    }

    pub async fn download_patcher(
        &self,
        install_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .download_exe_by_url(install_path, PATCHER_URL, PATCHER_EXE)
            .await?)
    }

    async fn download_exe_by_url(
        &self,
        install_path: &str,
        url: &str,
        exe: &str,
    ) -> Result<String, reqwest::Error> {
        let _ = create_dir_all(install_path);
        let path = install_path.to_string() + "\\" + exe;
        let _ = remove_file(&path);

        let response = reqwest::get(BASE_URL.to_string() + url).await?;
        let mut file = File::create(&path).unwrap();
        let mut content = Cursor::new(response.bytes().await?);
        let _ = copy(&mut content, &mut file);

        Ok(path)
    }
}
