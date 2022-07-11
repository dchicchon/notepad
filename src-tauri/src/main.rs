// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

mod modules;

use modules::{
  cmd::{db_insert, db_read},
  database::Database,
  dialog::{new_file, open_file, open_preferences, save_file},
};

use tauri::{
  CustomMenuItem, GlobalShortcutManager, Manager, Menu, MenuItem, RunEvent, Submenu, WindowBuilder,
};

use tauri_plugin_store::PluginBuilder;

// use std::{fs::File, io::Read};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences");
  let open = CustomMenuItem::new("open".to_string(), "Open...");
  let new = CustomMenuItem::new("new".to_string(), "New...");
  let save = CustomMenuItem::new("save".to_string(), "Save As...");

  let submenu1 = Submenu::new(
    "Notepad",
    Menu::new()
      .add_item(preferences)
      .add_native_item(MenuItem::Quit),
  );
  let submenu2 = Submenu::new(
    "File",
    Menu::new().add_item(open).add_item(save).add_item(new),
  );

  let menu = Menu::new().add_submenu(submenu1).add_submenu(submenu2);

  let app = tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .setup(|app| {
      WindowBuilder::new(
        app,
        "main",
        tauri::WindowUrl::App("src/main/index.html".into()),
      )
      .menu(menu)
      .build()?;
      Ok(())
    })
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![db_insert, db_read])
    .on_menu_event(|event| match event.menu_item_id() {
      "preferences" => {
        let handle = event.window().app_handle();
        open_preferences(&handle);
      }
      "open" => {
        // get handle
        let handle = event.window().app_handle();
        open_file(&handle);
      }
      "save" => {
        let handle = event.window().app_handle();
        save_file(&handle);
      }
      "new" => {
        println!("New menu item selected");
        let handle = event.window().app_handle();
        new_file(&handle);
      }
      _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error with app!");

  // app.run(ctx).expect("error while running tauri application");

  app.run(|app_handle, e| match e {
    // Application is ready (triggered only once)
    RunEvent::Ready => {
      let handle = app_handle.clone();
      let handle2 = app_handle.clone();
      app_handle
        .global_shortcut_manager()
        .register("CmdOrCtrl+S", move || {
          println!("Hotkey executed");
          // only open save dialog if there is no file path yet
          save_file(&handle);
        })
        .unwrap();
      app_handle
        .global_shortcut_manager()
        .register("CmdOrCtrl+O", move || {
          open_file(&handle2);
        })
        .unwrap();
    }

    // Triggered when a window is trying to close
    // Keep the event loop running even if all windows are closed
    // This allow us to catch system tray events when there is no window
    #[cfg(target_os = "windows")]
    RunEvent::ExitRequested { api, .. } => {
      println!("App is exiting");
    }
    #[cfg(target_os = "macos")]
    RunEvent::ExitRequested { api, .. } => {
      println!("App is exiting");
      api.prevent_exit();
    }
    _ => {}
  })
}
