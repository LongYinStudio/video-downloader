use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

fn main() {
    // 下载慢，开代理，再运行，但是事后必须关闭代理，不上会导致白屏
    download_yt_dlp().expect("Failed to download yt-dlp");
    tauri_build::build();
}

fn download_yt_dlp() -> Result<(), Box<dyn std::error::Error>> {
    // 确定目标路径
    let bin_dir = PathBuf::from("bin");
    fs::create_dir_all(&bin_dir).context("Failed to create bin directory")?;

    // 根据平台选择下载地址
    let (url, filename) = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", _) => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
            "my-yt-dlp-x86_64-pc-windows-msvc.exe",
        ),
        ("linux", "x86_64") => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux",
            "my-yt-dlp-x86_64-unknown-linux-gnu",
        ),
        // ("linux", "aarch64") => (
        //     "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux_aarch64",
        //     "my-yt-dlp-aarch64-unknown-linux-gnu",
        // ),
        ("macos", _) => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
            // "my-yt-dlp-universal-apple-darwin",
            "my-yt-dlp-aarch64-apple-darwin",
        ),
        (os, arch) => return Err(anyhow::anyhow!("Unsupported platform: {}-{}", os, arch).into()),
        // _ => return Err(format!("Unsupported platform: {}", std::env::consts::OS).into()),
    };

    let target_path = bin_dir.join(filename);

    // 如果文件已存在且有效则跳过下载
    if !is_valid_yt_dlp(&target_path) {
        println!("Downloading yt-dlp from: {}", url);

        // 带重试机制的下载
        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        let response = client
            .get(url)
            .send()
            .and_then(|r| r.error_for_status())
            .context("Failed to download yt-dlp")?;

        let content = response.bytes().context("Failed to read response body")?;

        // 原子写入文件
        let temp_path = target_path.with_extension("tmp");
        {
            let mut dest = File::create(&temp_path).context("Failed to create temp file")?;
            dest.write_all(&content)
                .context("Failed to write content")?;
        }

        // 设置文件权限（在移动文件之前）
        set_executable_permission(&temp_path).context("Failed to set permissions")?;

        // 重命名临时文件
        fs::rename(&temp_path, &target_path).context("Failed to move file")?;
    }

    // 强制验证文件
    verify_yt_dlp(&target_path).context("yt-dlp verification failed")?;

    // 添加构建依赖跟踪
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=YT_DLP_FORCE_UPDATE");

    Ok(())
}

fn set_executable_permission(path: &PathBuf) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))
            .context("Failed to set permissions")?;

        // 双重确认权限
        Command::new("chmod")
            .arg("+x")
            .arg(path)
            .status()
            .context("Failed to execute chmod")?;
    }
    Ok(())
}

fn is_valid_yt_dlp(path: &PathBuf) -> bool {
    // 通过环境变量强制更新
    if std::env::var("YT_DLP_FORCE_UPDATE").is_ok() {
        return false;
    }

    // 检查文件存在性和可执行性
    path.exists() && verify_yt_dlp(path).is_ok()
}

fn verify_yt_dlp(path: &PathBuf) -> Result<()> {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .context("Failed to execute yt-dlp")?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("cargo:warning=yt-dlp version: {}", version.trim());
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("yt-dlp validation failed: {}", error)
    }
}
