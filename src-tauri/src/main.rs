#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
fn refresh_app(window: tauri::Window) {
  println!("Refreshing application");
  window.eval("location.reload()").unwrap();
}

fn main() {
  tauri::Builder::default()
      .setup(|app| {
          let window = app.get_window("main").unwrap();
          window.set_title("Lemi AI - Customer Support Agent").unwrap();
          Ok(())
      })
      .invoke_handler(tauri::generate_handler![refresh_app])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
