use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<HashMap<String, Vec<String>>>>;

pub fn init_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}