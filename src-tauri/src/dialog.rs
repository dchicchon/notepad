use tauri::api::{dialog::FileDialogBuilder, Error};

pub fn open_file() {
    FileDialogBuilder::new()
        .add_filter("Text", &["txt"])
        .pick_file( |path_buf|  
        match path_buf {
            Some(p) => {
              let text = Some(tauri::api::file::read_string(p));
            }
            _ => {}
        });
}

pub fn save_file(text: String) {
    println!("Saving File");
    FileDialogBuilder::new().save_file(|path_buf| match path_buf {
        Some(p) => {
            println!("Saved File {:?}", p);
        }
        _ => {}
    });
}
