use anyhow::Result;

pub trait TagDataRepository {
    fn new() -> Self;
    fn init(&mut self, file_path: &str) -> Result<()>;
    fn save(&self) -> Result<()>;
    fn get_tag_data(&self, tag: &str) -> Option<String>;
    fn get_all_tags(&self) -> Vec<String>;
    fn get_all_tag_data(&self) -> Vec<String>;
    fn get_all_data(&self) -> Vec<(String, String)>;
    fn add_tag_data(&mut self, tag: String, command: String);
    fn remove_tag_data(&mut self, tag: &str);
    fn get_data_path(&self) -> String;
}
