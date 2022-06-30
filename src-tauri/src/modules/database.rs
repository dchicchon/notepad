
use std::{
    collections::HashMap, 
    sync::Mutex,
};

#[derive(Default)]
pub struct Database(pub Mutex<HashMap<String, String>>);