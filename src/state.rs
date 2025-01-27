use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref IPS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}
