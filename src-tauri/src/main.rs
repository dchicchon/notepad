#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use tauri::{
  api::dialog,
  api::dialog::FileDialogBuilder,
  CustomMenuItem,
  // GlobalShortcutManager,
  Manager,
  Menu,
  MenuItem,
  RunEvent,
  Submenu,
};

// use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
  let open = CustomMenuItem::new("open".to_string(), "Open...");
  let save_as = CustomMenuItem::new("save_as".to_string(), "Save As...");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

  // let tray_menu1 = SystemTrayMenu::new()
  //   .add_item(CustomMenuItem::new("toggle", "Toggle"))
  //   .add_item(CustomMenuItem::new("new", "New window"))
  //   .add_item(CustomMenuItem::new("icon_1", "Tray Icon 1"))
  //   .add_item(CustomMenuItem::new("icon_2", "Tray Icon 2"))
  //   .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
  //   .add_item(CustomMenuItem::new("exit_app", "Quit"));
  // let tray_menu2 = SystemTrayMenu::new()
  //   .add_item(CustomMenuItem::new("toggle", "Toggle"))
  //   .add_item(CustomMenuItem::new("new", "New window"))
  //   .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
  //   .add_item(CustomMenuItem::new("exit_app", "Quit"));
  // let is_menu1 = AtomicBool::new(true);

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
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "open" => {
        event.window().close().unwrap();
        FileDialogBuilder::new()
          .add_filter("Text", &["txt"])
          .pick_file(|path_buf| match path_buf {
            Some(p) => {
              let text = tauri::api::file::read_string(p);
              println!("{:?}", text);
            }
            _ => {}
          });
      }
      "save_as" => {
        println!("Saving File");
        FileDialogBuilder::new().save_file(|path_buf| match path_buf {
          Some(p) => {
            println!("Saved File {:?}", p);
          }
          _ => {}
        });
      }
      "close" => {
        event.window().close().unwrap();
      }
      "quit" => {
        std::process::exit(0);
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![cmd::log_operation])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  #[cfg(target_os = "macos")]
  // app.set_activation_policy(tauri::ActivationPolicy::Regular);
  app.run(|app_handle, e| match e {
    // Application is ready (triggered only once)
    RunEvent::Ready => {
      // let app_handle = app_handle.clone();
      // app_handle
      // .global_shortcut_manager()
      // .register("CmdOrCtrl+S", move || {
      //   let app_handle = app_handle.clone();
      //   let window = app_handle.get_window("main").unwrap();
      //   window.set_title("New title!").unwrap();
      // })
      // .unwrap();
    }

    // Triggered when a window is trying to close
    RunEvent::CloseRequested { label, api, .. } => {
      let app_handle = app_handle.clone();
      let window = app_handle.get_window(&label).unwrap();
      // use the exposed close api, and prevent the event loop to close
      api.prevent_close();
      // ask the user if he wants to quit
      dialog::ask(
        Some(&window),
        "Notepad",
        "Are you sure that you want to close Notepad?",
        move |answer| {
          if answer {
            // .close() cannot be called on the main thread
            std::thread::spawn(move || {
              app_handle.get_window(&label).unwrap().close().unwrap();
            });
          }
        },
      );
    }

    // Keep the event loop running even if all windows are closed
    // This allow us to catch system tray events when there is no window
    RunEvent::ExitRequested { api, .. } => {
      println!("App is exiting");
      api.prevent_exit();
    }
    _ => {}
  });
}
