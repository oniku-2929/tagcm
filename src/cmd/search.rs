use crate::repo::tag_data_repository::TagDataRepository;
pub fn search<T: TagDataRepository>(repo: T, search_str: String) {
    println!("Searching for tag {}", search_str);
    let all_tags = repo.get_all_tags();
    for tag in all_tags {
        if tag.starts_with(&search_str) {
            match repo.get_tag_data(&tag) {
                Some(cmd) => {
                    println!("{}: {}", tag, cmd);
                }
                None => println!("Command not found"),
            }
        }
    }
}
