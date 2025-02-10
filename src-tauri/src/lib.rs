// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn download(url: &str) -> String {
    // println!("{}", url);
    format!("下载地址 url:{}", url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
