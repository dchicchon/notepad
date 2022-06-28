// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

mod cmd;
mod dialog;

use tauri::{CustomMenuItem, GlobalShortcutManager, Manager, Menu, MenuItem, RunEvent, Submenu};

use cmd::{db_insert, db_read, Database};
use dialog::{open_file, save_file};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  let open = CustomMenuItem::new("open".to_string(), "Open...");
  let save_as = CustomMenuItem::new("save_as".to_string(), "Save As...");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

  let submenu = Submenu::new(
    "File",
    Menu::new()
      .add_item(open)
      .add_item(save_as)
      .add_item(close)
      .add_item(quit),
  );

  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_submenu(submenu);

  let app = tauri::Builder::default()
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![db_insert, db_read])
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "open" => {
        open_file(event);
      }
      "save" => {
        let handle = event.window().app_handle();
        save_file(&handle);
      }
      "save_as" => {
        let handle = event.window().app_handle();
        save_file(&handle);
      }
      "close" => {
        event.window().close().unwrap();
      }
      "quit" => {
        std::process::exit(0);
      }
      _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  // #[cfg(target_os = "macos")]
  app.run(|app_handle, e| match e {
    // Application is ready (triggered only once)
    RunEvent::Ready => {
      let app_handle = app_handle.clone();
      app_handle
        .global_shortcut_manager()
        .register("CmdOrCtrl+S", move || {
          println!("Hotkey executed");
          save_file(&app_handle);
        })
        .unwrap();
    }

    // Triggered when a window is trying to close
    // Keep the event loop running even if all windows are closed
    // This allow us to catch system tray events when there is no window
    RunEvent::ExitRequested { api, .. } => {
      println!("App is exiting");
      api.prevent_exit();
    }
    _ => {}
  });
}
