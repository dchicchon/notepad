use std::{
    fs::File,
    io::Write,
    collections::HashMap, 
    sync::Mutex
};

#[derive(Default)]
pub struct Database(pub Mutex<HashMap<String, String>>);

use tauri::{
    api::{dialog::FileDialogBuilder, file::read_string},
    AppHandle, Manager, State, WindowMenuEvent,
};

pub fn open_file(event: WindowMenuEvent) {
    println!("OpenFile");
    FileDialogBuilder::new()
        .add_filter("Text", &["txt"])
        .pick_file(move |path_buf| {
            match path_buf {
                Some(p) => {
                    let path = p.clone();
                    let name_path = path.clone();

                    let file_name = name_path.file_name().unwrap().to_str().unwrap().to_string();
                    let file_path = path.into_os_string().into_string().unwrap().to_string();
                    let value: String = read_string(p).unwrap().to_string();

                    let mut data = HashMap::new();
                    data.insert("path".to_string(), file_path);
                    data.insert("name".to_string(), file_name);
                    data.insert("text".to_string(), value);
                    let _result = event.window().emit("newFile", data);
                }
                _ => {}
            };
        });
}

pub fn save_file(handle: &AppHandle) {
    let app_handle = handle.clone();
    FileDialogBuilder::new().save_file(move |path_buf| match path_buf {
        Some(p) => {
            let state: State<Database> = app_handle.state();
            let text = state.0.lock().unwrap().get("text").cloned().unwrap();
            let mut file = File::create(&p).unwrap();
            let _result = file.write_all(&text.as_bytes());
        }
        _ => {}
    });
}
