use super::tag_data_repository::TagDataRepository;
use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct HashMapRepository {
    data: HashMap<String, String>,
    file_path: String,
}

impl TagDataRepository for HashMapRepository {
    fn new() -> Self {
        HashMapRepository {
            data: HashMap::new(),
            file_path: String::new(),
        }
    }

    fn init(&mut self, file_path: &str) -> Result<()> {
        self.file_path = file_path.to_string();
        let file = File::open(file_path)?;
        self.data = serde_json::from_reader(file)?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(&self.data)?;
        let mut file = File::create(self.file_path.as_str())?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn get_tag_data(&self, tag: &str) -> Option<String> {
        return self.data.get(tag).cloned();
    }

    fn get_all_tags(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    fn get_all_tag_data(&self) -> Vec<String> {
        self.data.values().cloned().collect()
    }

    fn get_all_data(&self) -> Vec<(String, String)> {
        self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn add_tag_data(&mut self, tag: String, command: String) {
        self.data.insert(tag, command);
        self.save().unwrap();
    }

    fn remove_tag_data(&mut self, tag: &str) {
        self.data.remove(tag);
    }
}
