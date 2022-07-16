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
    println!("OpenFile");
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
pub fn save_file(handle: &AppHandle) {
    let app_handle = handle.clone();
    let state: State<Database> = handle.state();
    let path = state.0.lock().unwrap().get("file").cloned();
    let unwrapped_path = get_path(path);
    // if path is not empty
    if unwrapped_path != "" {
        println!("There is a path buf!");
        // turn the path into a pathbuf
        let mut set_path = PathBuf::new();
        set_path.push(unwrapped_path);
        println!("the path: {:?}", set_path);
        let state: State<Database> = app_handle.state();
        let text = state.0.lock().unwrap().get("text").cloned().unwrap();
        println!("the text to save: {}", text);
        let mut file = File::create(set_path).unwrap();
        let text_bytes = text.as_bytes();
        let _result = file.write_all(text_bytes);
        return;
    }
    FileDialogBuilder::new()
        .add_filter("Text", &["txt"])
        .save_file(move |path_buf| match path_buf {
            Some(p) => {
                // println!("the path: {:?}", p);
                let path = p.clone();
                let name_path = path.clone();

                let file_name = name_path.file_name().unwrap().to_str().unwrap().to_string();
                let file_path = path.into_os_string().into_string().unwrap().to_string();

                // println!("New file path: {}", file_path);
                let state: State<Database> = app_handle.state();
                let text = state.0.lock().unwrap().get("text").cloned().unwrap();
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
    println!("Open preferences");

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
        println!("Got event from preferences window, emit to main");
        let payload = event.payload().unwrap(); // turn this into a hashmap?
        println!("Payload: {:?}", payload);
        let mut data = HashMap::new();
        data.insert("setting", payload);
        let _result = main_window.emit("state_change", data);
    });
}

#[tauri::command]
pub fn install_font(handle: &AppHandle) {
    println!("OpenFile");
    let new_handle = handle.clone();
    FileDialogBuilder::new()
        .add_filter("Font File", &["ttf"])
        .pick_file(move |path_buf| {
            match path_buf {
                Some(p) => {
                    let path = p.clone();
                    let name_path = path.clone();

                    let file_path = path.into_os_string().into_string().unwrap().to_string();
                    let file_name = name_path.file_name().unwrap().to_str().unwrap().to_string();

                    // make a copy of the font file in this app directory
                    

                }
                _ => {}
            };
        });
}
