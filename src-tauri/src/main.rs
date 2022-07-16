#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;

mod modules;

use modules::{
  cmd::{db_insert, db_read, get_font_families},
  database::Database,
  dialog::{new_file, open_file, open_preferences, save_file},
};

use tauri::{
  CustomMenuItem, GlobalShortcutManager, Manager, Menu, MenuItem, RunEvent, Submenu, WindowBuilder,
  WindowEvent,
};

use tauri_plugin_store::PluginBuilder;

// use std::{fs::File, io::Read};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  env::set_var("RUST_BACKTRACE", "1");

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
  let submenu3 = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste),
  );

  let windows_menu = Menu::new()
    .add_submenu(submenu1)
    .add_submenu(submenu2)
    .add_submenu(submenu3);
  let mac_menu = windows_menu.clone();
  #[cfg(target_os = "macos")]
  let app = tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .menu(mac_menu)
    .on_menu_event(move |event| match event.menu_item_id() {
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
    .setup(|app| {
      let _window = WindowBuilder::new(
        app,
        "main",
        tauri::WindowUrl::App("notepad.html".into()),
      )
      .build();
      Ok(())
    })
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![db_insert, db_read, get_font_families])
    .build(tauri::generate_context!())
    .expect("error with app!");
  #[cfg(target_os = "windows")]
  let app = tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .setup(|app| {
      let window = WindowBuilder::new(
        app,
        "main",
        tauri::WindowUrl::App("notepad.html".into()),
      )
      .menu(windows_menu)
      .build()?;

      let window_2 = window.clone();
      window.on_menu_event(move |event| match event.menu_item_id() {
        "preferences" => {
          let handle = window_2.app_handle();
          open_preferences(&handle);
        }
        "open" => {
          // get handle
          let handle = window_2.app_handle();
          open_file(&handle);
        }
        "save" => {
          let handle = window_2.app_handle();
          save_file(&handle);
        }
        "new" => {
          println!("New menu item selected");
          let handle = window_2.app_handle();
          new_file(&handle);
        }
        _ => {}
      });
      Ok(())
    })
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![db_insert, db_read])
    .build(tauri::generate_context!())
    .expect("error with app!");

  app.run(|app_handle, e| match e {
    // Application is ready (triggered only once)
    RunEvent::Ready => {
      let handle = app_handle.clone();
      let handle2 = app_handle.clone();
      let handle3 = app_handle.clone();
      app_handle
        .global_shortcut_manager()
        .register("CmdOrCtrl+S", move || {
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
      app_handle
        .global_shortcut_manager()
        .register("CmdOrCtrl+N", move || {
          new_file(&handle3);
        })
        .unwrap();
    }

    RunEvent::WindowEvent {
      label,
      event: WindowEvent::CloseRequested { api, .. },
      ..
    } => {
      println!("Label type: {}", label);
      #[cfg(target_os = "macos")]
      if label == "main" {
        let window = app_handle.get_window("main").unwrap();
        api.prevent_close();
        window.hide().unwrap();
      }
    }

    RunEvent::ExitRequested { api, .. } => {
      println!("App is exiting");
      #[cfg(target_os = "macos")]
      api.prevent_exit();
    }
    _ => {}
  })
}
