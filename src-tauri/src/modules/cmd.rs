use crate::Database;
use std::fs::read_dir;
use std::env;
use tauri::State;
use font_loader::system_fonts;

#[tauri::command]
pub fn db_insert(key: String, value: String, db: State<Database>) {
  db.0.lock().unwrap().insert(key, value);
}

#[tauri::command]
pub fn db_read(key: String, db: State<Database>) -> Option<String> {
  db.0.lock().unwrap().get(&key).cloned()
}

#[tauri::command]
pub fn get_fonts(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
  let fonts = system_fonts::query_all();
  Ok(fonts)
}
