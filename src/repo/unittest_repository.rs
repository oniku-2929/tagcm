use super::tag_data_repository::TagDataRepository;
use anyhow::{Ok, Result};
use std::collections::HashMap;

pub struct UnitTestRepository {
    data: HashMap<String, String>,
}

impl TagDataRepository for UnitTestRepository {
    fn new() -> Self {
        UnitTestRepository {
            data: HashMap::new(),
        }
    }

    fn init(&mut self, file_path: &str) -> Result<()> {
        _ = file_path;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        Ok(())
    }

    fn get_tag_data(&self, tag: &str) -> Option<String> {
        return self.data.get(tag).cloned();
    }

    fn get_all_tags(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
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
