fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动失败");
}
