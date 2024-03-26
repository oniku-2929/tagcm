use crate::repo::tag_data_repository::TagDataRepository;

pub fn delete<T: TagDataRepository>(mut repo: &T, tag: String) {
    repo.remove_tag_data(&tag);
    match repo.save() {
        Ok(_) => println!("Tag deleted"),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::hashmap_repository::HashMapRepository;
    #[test]
    fn test_delete() {
        let repo = &HashMapRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        delete(repo, "test".to_string());
        assert_eq!(repo.get_tag_data("test"), None);
    }
}
