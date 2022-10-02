use std::sync::Mutex;

use once_cell::sync::Lazy;

pub struct Photo {
    pub name: String,
    pub description: String,
}

pub static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![]));
