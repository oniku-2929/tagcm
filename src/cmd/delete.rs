use crate::repo::tag_data_repository::TagDataRepository;

pub fn delete<T: TagDataRepository>(repo: &mut T, tag: String) {
    repo.remove_tag_data(&tag);
    match repo.save() {
        Ok(_) => println!("Tag deleted"),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::unittest_repository::UnitTestRepository;
    #[test]
    fn test_delete() {
        let mut repo = UnitTestRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        delete(&mut repo, "test".to_string());
        assert_eq!(repo.get_tag_data("test"), None);
    }
}
