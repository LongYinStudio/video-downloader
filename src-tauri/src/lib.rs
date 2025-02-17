// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use directories::UserDirs;
use std::env;
use std::str;
use tauri::Emitter;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

// 定义一个名为 get_download_dir 的函数，该函数返回一个 Result 类型，其中包含一个 String 或一个 String 错误信息
fn get_download_dir() -> Result<String, String> {
    // 尝试创建一个新的 UserDirs 实例，如果失败则返回一个包含错误信息的 Result
    let user_dirs = UserDirs::new().ok_or("无法获取用户目录".to_string())?;
    // 尝试获取用户的下载目录，如果失败则返回一个包含错误信息的 Result
    let download_dir = user_dirs
        .download_dir()
        .ok_or("无法获取下载目录".to_string())?;
    // 将下载目录的路径转换为字符串并返回，如果转换失败则解引用并转换为字符串
    Ok(download_dir.to_str().unwrap().to_string())
}

#[tauri::command]
async fn download(url: &str, app: tauri::AppHandle) -> Result<(), String> {
    println!("{}", url);
    // 获取系统下载目录
    let download_dir = get_download_dir()?;
    println!("Download directory: {}", download_dir);

    // `sidecar()` 只需要文件名, 不像 JavaScript 中的整个路径
    // format!("下载地址 url:{}", url);
    let output = format!("{}/%(title)s.%(ext)s", download_dir);
    let sidecar_command = app.shell().sidecar("my-yt-dlp").unwrap().args([
        // "--proxy",
        // "127.0.0.1:7890",
        "-o", &output, &url,
    ]);
    let (mut _rx, mut _child) = sidecar_command.spawn().expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        while let Some(event) = _rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    // 处理标准输出
                    // let line_str = match str::from_utf8(&line) {
                    //     Ok(v) => v,
                    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    // };
                    // println!("yt-dlp stdout: {}", line_str);
                    let line_str = String::from_utf8_lossy(&line);
                    println!("yt-dlp stdout: {}", line_str);

                    // 发送实时输出到前端
                    app.emit("yt-dlp-progress", line_str).unwrap();
                }
                CommandEvent::Stderr(line) => {
                    // 处理错误输出
                    // let line_str = match str::from_utf8(&line) {
                    //     Ok(v) => v,
                    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    // };
                    // eprintln!("yt-dlp stderr: {}", line_str);
                    let line_str = String::from_utf8_lossy(&line);
                    eprintln!("yt-dlp stderr: {}", line_str);

                    // 发送错误信息到前端
                    app.emit("yt-dlp-error", line_str).unwrap();
                }
                CommandEvent::Error(err) => {
                    // 处理错误
                    eprintln!("yt-dlp error: {}", err);

                    // 发送错误信息到前端
                    app.emit("yt-dlp-error", err.to_string()).unwrap();
                }
                _ => {}
            }
        }
    })
    .await
    .expect("Execution failed!");

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
