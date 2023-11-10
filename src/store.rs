use std::collections::HashMap;
use std::error::Error;

pub trait Store {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String) -> Result<String, Box<dyn Error>>;
    fn remove(&mut self, key: &str) -> Result<String, Box<dyn Error>>;
    fn list(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn get_last_id(&self) -> Result<i32, Box<dyn Error>> {
        let keys = self.list()?;
        let mut last_id = 0;
        for key in keys {
            let id = key.parse::<i32>().unwrap();
            if id > last_id {
                last_id = id;
            }
        }
        Ok(last_id)
    }
}

pub struct LocalStore {
    store: HashMap<String, String>,
}

impl LocalStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Store for LocalStore {
    fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: String) -> Result<String, Box<dyn Error>> {
        self.store.insert(key.to_string(), value);
        Ok("OK".to_string())
    }

    fn remove(&mut self, key: &str) -> Result<String, Box<dyn Error>> {
        match self.store.remove(key) {
            Some(_) => Ok("OK".to_string()),
            None => Err("Key not found".into()),
        }
    }

    fn list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut keys: Vec<String> = self.store.keys().cloned().collect();
        keys.sort();
        Ok(keys)
    }
}

pub struct FileStore {
    path: String,
}

impl FileStore {
    pub fn new(path: String) -> Self {
        // Create file if it doesn't exist
        if !std::path::Path::new(&path).exists() {
            println!("Creating file: {}", &path);
            std::fs::write(&path, "{}").unwrap();
        }
        Self { path }
    }
}

impl Store for FileStore {
    fn get(&self, key: &str) -> Option<String> {
        let content = std::fs::read_to_string(&self.path).unwrap();
        let store: HashMap<String, String> = serde_json::from_str(&content).unwrap();
        store.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: String) -> Result<String, Box<dyn Error>> {
        let mut store: HashMap<String, String> = match std::fs::read_to_string(&self.path) {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                println!("Error reading file: {}", e);
                HashMap::new()
            }
        };
        store.insert(key.to_string(), value);
        let content = serde_json::to_string(&store).unwrap();
        std::fs::write(&self.path, content)?;
        Ok("OK".to_string())
    }

    fn remove(&mut self, key: &str) -> Result<String, Box<dyn Error>> {
        let mut store: HashMap<String, String> = match std::fs::read_to_string(&self.path) {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(_) => HashMap::new(),
        };
        match store.remove(key) {
            Some(_) => {
                let content = serde_json::to_string(&store).unwrap();
                std::fs::write(&self.path, content)?;
                Ok("OK".to_string())
            }
            None => Err("Key not found".into()),
        }
    }

    fn list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let content = std::fs::read_to_string(&self.path).unwrap();
        let mut store: HashMap<String, String> = serde_json::from_str(&content).unwrap();
        let mut keys: Vec<String> = store.keys().cloned().collect();
        keys.sort();
        Ok(keys)
    }
}
