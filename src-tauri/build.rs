use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn main() {
    // 下载慢，开代理，再运行，但是事后必须关闭代理，否则会导致白屏
    download_yt_dlp().expect("Failed to download yt-dlp");
    tauri_build::build();
}

fn download_yt_dlp() -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = PathBuf::from("bin");
    fs::create_dir_all(&bin_dir).context("Failed to create bin directory")?;

    let target = std::env::var("TARGET").unwrap_or_default();
    let is_macos = std::env::consts::OS == "macos" || target.contains("apple-darwin");

    if is_macos {
        // macOS 下 yt-dlp 官方提供的是 universal 二进制 (包含 x86_64 与 arm64)
        // Tauri 在构建 universal-apple-darwin 时，会分别编译 x86_64 与 aarch64，因此需要对应命名的 sidecar 文件
        let macos_targets = [
            "my-yt-dlp-aarch64-apple-darwin",
            "my-yt-dlp-x86_64-apple-darwin",
            "my-yt-dlp-universal-apple-darwin",
        ];

        let need_download = macos_targets
            .iter()
            .any(|name| !is_valid_yt_dlp(&bin_dir.join(name)));

        if need_download {
            let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos";
            println!("Downloading yt-dlp for macOS from: {}", url);

            let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
            let response = client
                .get(url)
                .send()
                .and_then(|r| r.error_for_status())
                .context("Failed to download yt-dlp for macOS")?;

            let content = response.bytes().context("Failed to read response body")?;

            let temp_path = bin_dir.join("yt-dlp_macos.tmp");
            {
                let mut dest = File::create(&temp_path).context("Failed to create temp file")?;
                dest.write_all(&content)
                    .context("Failed to write content")?;
            }
            set_executable_permission(&temp_path).context("Failed to set permissions")?;

            for target_name in &macos_targets {
                let dest_path = bin_dir.join(target_name);
                fs::copy(&temp_path, &dest_path)
                    .with_context(|| format!("Failed to copy to {:?}", dest_path))?;
                set_executable_permission(&dest_path).ok();
            }

            let _ = fs::remove_file(&temp_path);
        }

        // 验证文件有效性
        for target_name in &macos_targets {
            let dest_path = bin_dir.join(target_name);
            if let Err(e) = verify_yt_dlp(&dest_path) {
                // 如果当前平台架构不匹配无法执行，仅打印警告信息
                println!("cargo:warning=Note verifying {}: {}", target_name, e);
            }
        }
    } else {
        // Windows 或 Linux 平台
        let (url, filename) = match (std::env::consts::OS, std::env::consts::ARCH) {
            ("windows", _) => (
                "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
                "my-yt-dlp-x86_64-pc-windows-msvc.exe",
            ),
            ("linux", "aarch64") => (
                "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux_aarch64",
                "my-yt-dlp-aarch64-unknown-linux-gnu",
            ),
            ("linux", _) => (
                "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux",
                "my-yt-dlp-x86_64-unknown-linux-gnu",
            ),
            (os, arch) => return Err(anyhow::anyhow!("Unsupported platform: {}-{}", os, arch).into()),
        };

        let target_path = bin_dir.join(filename);

        if !is_valid_yt_dlp(&target_path) {
            println!("Downloading yt-dlp from: {}", url);

            let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
            let response = client
                .get(url)
                .send()
                .and_then(|r| r.error_for_status())
                .context("Failed to download yt-dlp")?;

            let content = response.bytes().context("Failed to read response body")?;

            let temp_path = target_path.with_extension("tmp");
            {
                let mut dest = File::create(&temp_path).context("Failed to create temp file")?;
                dest.write_all(&content)
                    .context("Failed to write content")?;
            }

            set_executable_permission(&temp_path).context("Failed to set permissions")?;
            fs::rename(&temp_path, &target_path).context("Failed to move file")?;
        }

        verify_yt_dlp(&target_path).context("yt-dlp verification failed")?;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=YT_DLP_FORCE_UPDATE");

    Ok(())
}

fn set_executable_permission(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));

        Command::new("chmod")
            .arg("+x")
            .arg(path)
            .status()
            .ok();
    }
    Ok(())
}

fn is_valid_yt_dlp(path: &Path) -> bool {
    if std::env::var("YT_DLP_FORCE_UPDATE").is_ok() {
        return false;
    }

    if !path.exists() {
        return false;
    }

    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() < 1024 * 1024 {
            return false;
        }
        if verify_yt_dlp(path).is_ok() {
            return true;
        }
        // 如果当前主机无法直接执行（如跨架构编译），但存在且有执行权限和有效大小，也认为有效
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                return true;
            }
        }
        #[cfg(not(unix))]
        {
            return true;
        }
    }

    false
}

fn verify_yt_dlp(path: &Path) -> Result<()> {
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
