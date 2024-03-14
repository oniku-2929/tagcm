use crate::repo::tag_data_repository::TagDataRepository;

pub fn delete<T: TagDataRepository>(mut repo: T, tag: String) {
    repo.remove_tag_data(&tag);
    match repo.save() {
        Ok(_) => println!("Tag deleted"),
        Err(e) => println!("Error: {}", e),
    }
}
