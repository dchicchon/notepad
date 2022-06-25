

use tauri::{ 
  State
};

use std::{
  collections::HashMap,
  sync::{
  Mutex
  },
};

#[derive(Default)]
pub struct Database(pub Mutex<HashMap<String, String>>);

#[tauri::command]
pub fn db_insert(key: String, value: String, db: State<Database>) {
  db.0.lock().unwrap().insert(key,value);
}

#[tauri::command]
pub fn db_read(key: String, db: State<Database>) -> Option<String> {
  db.0.lock().unwrap().get(&key).cloned()
}