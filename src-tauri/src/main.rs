use tauri::Manager;

// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

#[derive(serde::Deserialize)]
struct EventType {
  message: String,
}


fn main() {
  tauri::Builder::default()
    .setup(|app| {
      app.listen_global("click", |event| {
        if let Some(payload) = event.payload() {
          let variant1: EventType = serde_json::from_str(payload).unwrap();

          // Print var1
          println!("msg: {}", variant1.message);
        }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
