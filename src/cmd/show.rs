use crate::repo::tag_data_repository::TagDataRepository;
pub fn show_all<T: TagDataRepository>(repo: &T) {
    repo.get_all_data().iter().for_each(|(tag, data)| {
        println!("{}: {}", tag, data);
    });
}

pub fn show<T: TagDataRepository>(repo: &T, tag: String) {
    match repo.get_tag_data(&tag) {
        Some(cmd) => println!("{}: {}", tag, cmd),
        None => println!("Command not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::hashmap_repository::HashMapRepository;
    #[test]
    fn test_show() {
        let repo = &HashMapRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        show(repo, "test".to_string());
        show(repo, "test2".to_string());
    }

    #[test]
    fn test_show_all() {
        let repo = &HashMapRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        repo.add_tag_data("test2".to_string(), "echo test2".to_string());
        show_all(repo);
    }
}
