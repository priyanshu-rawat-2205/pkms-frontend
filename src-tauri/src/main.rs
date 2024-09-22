// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  api::process::{Command, CommandEvent},
  Manager,
};

fn main() {
  tauri::Builder::default()
  .setup(|app| {
    let window = app.get_window("main").unwrap();
    tauri::async_runtime::spawn(async move {
      let (mut rx, mut child) = Command::new_sidecar("pkms") // This is only the name of the binary itself, not the complete path like on the javascript side above
        .expect("failed to setup `pkms` sidecar")
        .spawn()
        .expect("Failed to spawn packaged node");
    });

    Ok(())
  })
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
