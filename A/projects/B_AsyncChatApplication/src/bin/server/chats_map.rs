use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::chats::Chat;

pub struct ChatTracker(Mutex<HashMap<String, Arc<Chat>>>); 

impl ChatTracker {
    pub fn new() -> ChatTracker {
        ChatTracker(Mutex::new(HashMap::new()))
    }

    pub fn find(&self, name: &string) -> Option<Arc<Chats>>{
        self.0.lock().unwrap().get(name).cloned();


    }

    pub fn find_or_new(&self, name: Arc<String>) -> Arc<Chats> {
        self.0.lock.unwrap().entry(name.clone())
        .or_insert_with(|| Arc::new(Chats::new)).clone()
    }

    
}
    