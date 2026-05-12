// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use directories::UserDirs;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::Emitter;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

#[derive(Default)]
struct DownloadState {
    child: Mutex<Option<CommandChild>>,
    cancelled: AtomicBool,
}

// 定义一个名为 get_download_dir 的函数，该函数返回一个 Result 类型，其中包含一个 String 或一个 String 错误信息
fn get_download_dir() -> Result<String, String> {
    // 尝试创建一个新的 UserDirs 实例，如果失败则返回一个包含错误信息的 Result
    let user_dirs = UserDirs::new().ok_or("无法获取用户目录".to_string())?;
    // 尝试获取用户的下载目录，如果失败则返回一个包含错误信息的 Result
    let download_dir = user_dirs
        .download_dir()
        .ok_or("无法获取下载目录".to_string())?;
    // 将下载目录的路径转换为字符串并返回
    Ok(download_dir
        .to_str()
        .ok_or("下载目录包含无效字符".to_string())?
        .to_string())
}

fn get_format_args(format_preset: &str) -> Result<Vec<String>, String> {
    let args = match format_preset {
        "" | "best" => Vec::new(),
        "1080p" => vec![
            "-f".to_string(),
            "bv*[height<=1080]+ba/b[height<=1080]/best[height<=1080]".to_string(),
        ],
        "720p" => vec![
            "-f".to_string(),
            "bv*[height<=720]+ba/b[height<=720]/best[height<=720]".to_string(),
        ],
        "480p" => vec![
            "-f".to_string(),
            "bv*[height<=480]+ba/b[height<=480]/best[height<=480]".to_string(),
        ],
        "audio" => vec![
            "-x".to_string(),
            "--audio-format".to_string(),
            "mp3".to_string(),
            "-f".to_string(),
            "bestaudio/best".to_string(),
        ],
        _ => return Err("不支持的下载格式".to_string()),
    };

    Ok(args)
}

// 测试
// 国内：https://www.bilibili.com/video/BV1GzfUYmEGE
// 国外：https://www.youtube.com/watch?v=ObEN8jqJZ7o
#[tauri::command]
async fn download(
    url: &str,
    dir: &str,
    proxy: &str,
    format_preset: &str,
    app: tauri::AppHandle,
    state: tauri::State<'_, DownloadState>,
) -> Result<(), String> {
    {
        let child = state
            .child
            .lock()
            .map_err(|_| "下载状态锁定失败".to_string())?;
        if child.is_some() {
            return Err("已有下载任务正在运行".to_string());
        }
    }

    // 获取系统下载目录
    let default_download_dir = get_download_dir()?;
    // 如果用户指定了目录，则使用用户选择的目录；否则使用默认下载目录
    let download_dir = if !dir.is_empty() {
        dir.to_string()
    } else {
        default_download_dir
    };
    println!("Download directory: {}", download_dir);
    let output = format!("{}/%(title)s.%(ext)s", download_dir);

    let mut args = get_format_args(format_preset)?;

    // 设置代理
    // if !proxy.is_empty() {
    //     println!("有代理{}", proxy.to_string());
    // } else {
    //     println!("无代理");
    // }
    if !proxy.is_empty() {
        args.push("--proxy".to_string());
        args.push(proxy.to_string());
    }
    args.push("-o".to_string());
    args.push(output);
    args.push(url.to_string());

    let args_string = args.join(" ");
    println!("完整命令：yt-dlp {}", args_string);
    // `sidecar()` 只需要文件名, 不像 JavaScript 中的整个路径
    let sidecar_command = app
        .shell()
        .sidecar("my-yt-dlp")
        .map_err(|err| format!("无法加载 yt-dlp sidecar：{}", err))?
        .args(&args);
    let (mut rx, _child) = sidecar_command
        .spawn()
        .map_err(|err| format!("无法启动 yt-dlp：{}", err))?;

    state.cancelled.store(false, Ordering::SeqCst);
    {
        let mut child = state
            .child
            .lock()
            .map_err(|_| "下载状态锁定失败".to_string())?;
        *child = Some(_child);
    }

    let mut last_error = String::new();
    let mut exit_code = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                let line_str = String::from_utf8_lossy(&line).to_string();
                println!("yt-dlp stdout: {}", line_str);

                if let Err(err) = app.emit("yt-dlp-progress", line_str) {
                    eprintln!("emit yt-dlp-progress failed: {}", err);
                }
            }
            CommandEvent::Stderr(line) => {
                let line_str = String::from_utf8_lossy(&line).to_string();
                eprintln!("yt-dlp stderr: {}", line_str);
                last_error = line_str.clone();

                if let Err(err) = app.emit("yt-dlp-error", line_str) {
                    eprintln!("emit yt-dlp-error failed: {}", err);
                }
            }
            CommandEvent::Error(err) => {
                let message = err.to_string();
                eprintln!("yt-dlp error: {}", message);
                last_error = message.clone();

                if let Err(err) = app.emit("yt-dlp-error", message) {
                    eprintln!("emit yt-dlp-error failed: {}", err);
                }
            }
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                let mut child = state
                    .child
                    .lock()
                    .map_err(|_| "下载状态锁定失败".to_string())?;
                *child = None;
            }
            _ => {}
        }
    }

    {
        let mut child = state
            .child
            .lock()
            .map_err(|_| "下载状态锁定失败".to_string())?;
        *child = None;
    }

    if state.cancelled.swap(false, Ordering::SeqCst) {
        return Err("下载已取消".to_string());
    }

    if exit_code != Some(0) {
        let message = if last_error.trim().is_empty() {
            match exit_code {
                Some(code) => format!("yt-dlp 退出异常，退出码：{}", code),
                None => "yt-dlp 已终止，但未返回退出码".to_string(),
            }
        } else {
            last_error
        };

        return Err(message);
    }

    Ok(())
}

#[tauri::command]
fn cancel_download(state: tauri::State<'_, DownloadState>) -> Result<(), String> {
    state.cancelled.store(true, Ordering::SeqCst);

    let child = {
        let mut child = state
            .child
            .lock()
            .map_err(|_| "下载状态锁定失败".to_string())?;
        child.take()
    };

    match child {
        Some(child) => child.kill().map_err(|err| format!("取消下载失败：{}", err)),
        None => {
            state.cancelled.store(false, Ordering::SeqCst);
            Err("当前没有正在下载的任务".to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DownloadState::default())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![download, cancel_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
