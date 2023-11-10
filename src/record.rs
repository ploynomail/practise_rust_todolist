use crate::cli::Commands;
use crate::store::Store;
pub struct Record {
    id: i32,
    content: String,
}

impl Record {
    pub fn new(id: i32, content: String) -> Self {
        Self { id, content }
    }
}

pub struct Command<'a> {
    store: &'a mut Box<dyn Store>,
    command: &'a Commands,
}

impl<'a> Command<'a> {
    pub fn new(command: &'a Commands, store: &'a mut Box<dyn Store>) -> Self {
        Self {
            store,
            command: command,
        }
    }
    pub fn info(&self) -> String {
        format!("Command: todo list")
    }

    pub fn add(&mut self, record: Record) -> String {
        let last_id = self.store.get_last_id().unwrap() + 1;
        let result = self
            .store
            .set(last_id.to_string().as_str(), record.content.clone())
            .unwrap();
        format!("Add: {}", result)
    }

    pub fn get(&self, key: &str) -> String {
        match self.store.get(key) {
            Some(value) => format!("Get: {}", value),
            None => format!("Get: No value found for key {}", key),
        }
    }
    pub fn remove(&mut self, key: &str) -> String {
        match self.store.remove(key) {
            Ok(value) => format!("Remove: {}", value),
            Err(e) => format!("Remove: {}", e),
        }
    }
    pub fn list(&self) -> String {
        match self.store.list() {
            Ok(value) => format!("List: {:?}", value),
            Err(e) => format!("List: {}", e),
        }
    }
    pub fn execute(&mut self) -> String {
        match self.command {
            Commands::Info => self.info(),
            Commands::Add { content } => self.add(Record::new(1, content.clone().unwrap())),
            Commands::Get { id } => self.get(&id.unwrap().to_string()),
            Commands::Remove { id } => self.remove(&id.unwrap().to_string()),
            Commands::List => self.list(),
        }
    }
}
