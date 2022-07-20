use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use tauri::{
    api::{dialog::FileDialogBuilder, file::read_string},
    window::WindowBuilder,
    AppHandle, Manager, State,
};

use crate::Database;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// dialog to open new file. might consider creating new windows in the future
pub fn open_file(handle: &AppHandle) {
    let new_handle = handle.clone();
    FileDialogBuilder::new()
        .add_filter("Text", &["txt"])
        .pick_file(move |path_buf| {
            match path_buf {
                Some(p) => {
                    let path = p.clone();
                    let name_path = path.clone();

                    let file_path = path.into_os_string().into_string().unwrap().to_string();
                    let file_name = name_path.file_name().unwrap().to_str().unwrap().to_string();
                    let value: String = read_string(p).unwrap().to_string();

                    let mut data = HashMap::new();
                    data.insert("path".to_string(), file_path);
                    data.insert("name".to_string(), file_name);
                    data.insert("text".to_string(), value);
                    let _result = new_handle.emit_all("state_change", data); // TODO: check for errors
                }
                _ => {}
            };
        });
}

// unwrapping path helper
fn get_path(option: Option<String>) -> String {
    match option {
        Some(inner) => inner,
        None => String::from(""),
    }
}

// save file. if the path is the same then just overwrite the previous and no dialog
pub fn save_file(handle: &AppHandle, window_closing: Option<bool>) {
    let app_handle = handle.clone();
    let state: State<Database> = handle.state();
    let path = state.0.lock().unwrap().get("file").cloned();
    let unwrapped_path = get_path(path);

    // if file has been saved before
    if unwrapped_path != "" {
        // turn the path into a pathbuf
        let mut set_path = PathBuf::new();
        set_path.push(unwrapped_path);
        let state: State<Database> = app_handle.state();
        let text = state.0.lock().unwrap().get("text").cloned().unwrap();
        let mut file = File::create(set_path).unwrap();
        let text_bytes = text.as_bytes();
        let _result = file.write_all(text_bytes);
        return;
    }
    // Check if window is closing
    if window_closing.unwrap() {
        FileDialogBuilder::new()
            .add_filter("Text", &["txt"])
            .save_file(move |path_buf| match path_buf {
                Some(p) => {
                    let path = p.clone();

                    let file_path = path.into_os_string().into_string().unwrap().to_string();

                    let state: State<Database> = app_handle.state();
                    let text = state.0.lock().unwrap().get("text").cloned().unwrap(); // getting text from our database
                    let text_bytes = text.as_bytes();

                    let mut file = File::create(&file_path).unwrap();
                    let _result = file.write_all(&text_bytes);

                    let main_window = app_handle.get_window("main").unwrap();
                    let _result = main_window.close();
                }
                _ => {}
            });
    } else {
        FileDialogBuilder::new()
            .add_filter("Text", &["txt"])
            .save_file(move |path_buf| match path_buf {
                Some(p) => {
                    let path = p.clone();
                    let name_path = path.clone();

                    let file_name = name_path.file_name().unwrap().to_str().unwrap().to_string();
                    let file_path = path.into_os_string().into_string().unwrap().to_string();

                    let state: State<Database> = app_handle.state();
                    let text = state.0.lock().unwrap().get("text").cloned().unwrap(); // getting text from our database
                    let text_bytes = text.as_bytes();

                    let mut file = File::create(&file_path).unwrap();
                    let _result = file.write_all(&text_bytes);

                    let mut data = HashMap::new();
                    data.insert("path".to_string(), file_path);
                    data.insert("name".to_string(), file_name);
                    let _result = app_handle.emit_all("state_change", data); // TODO: check for errors
                }
                _ => {}
            });
    }
}

// opening a new file
pub fn new_file(handle: &AppHandle) {
    let mut data = HashMap::new();
    data.insert("path".to_string(), "");
    data.insert("name".to_string(), "Untitled");
    data.insert("text".to_string(), " ");
    let main_window = handle.get_window("main").unwrap();
    main_window.show().unwrap();
    let _result = handle.emit_all("state_change", data);
}

// helper to check if a window exists
fn window_exists(label: &str, handle: &AppHandle) -> bool {
    let window = handle.get_window(label);
    match window {
        Some(_window) => true,
        None => false,
    }
}

// open preferences window
pub fn open_preferences(handle: &AppHandle) {
    let main_window = handle.get_window("main").unwrap();
    let test_handle = handle.clone();
    if window_exists("preferences", &test_handle) {
        return;
    };
    // how to make sure this window does not have menu?
    let preferences_window = WindowBuilder::new(
        handle,
        "preferences",
        tauri::WindowUrl::App("preferences.html".into()),
    )
    .center()
    .inner_size(300.0, 250.0)
    .max_inner_size(300.0, 250.0)
    .always_on_top(true)
    .resizable(false)
    .build()
    .unwrap();

    // make sure theres no menu for preferences?

    let _id = preferences_window.listen("update-setting", move |event| {
        let payload = event.payload().unwrap(); // turn this into a hashmap?
        let mut data = HashMap::new();
        data.insert("setting", payload);
        let _result = main_window.emit("state_change", data);
    });
}
