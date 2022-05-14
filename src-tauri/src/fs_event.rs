
// #[serde::{Deserialize}]
#[derive(serde::Deserialize)]
pub struct DataType {
  msg1: String,
  msg2: String
}

#[tauri::command]
pub fn my_custom_command(data: DataType) -> String {
  println!("I was invoked from JS! {} {}", data.msg1, data.msg2);
  return "test".to_string()
}