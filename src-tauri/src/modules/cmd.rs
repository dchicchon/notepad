
use tauri::{ 
  State
};

use crate::Database;

#[tauri::command]
pub fn db_insert(key: String, value: String, db: State<Database>) {
  // println!("Key: {}", key);
  // println!("Value: {}", value);
  db.0.lock().unwrap().insert(key,value);
}

#[tauri::command]
pub fn db_read(key: String, db: State<Database>) -> Option<String> {
  db.0.lock().unwrap().get(&key).cloned()
}