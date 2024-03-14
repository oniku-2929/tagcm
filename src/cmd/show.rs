use crate::repo::tag_data_repository::TagDataRepository;
pub fn show_all<T: TagDataRepository>(repo: T) {
    repo.get_all_data().iter().for_each(|(tag, data)| {
        println!("{}: {}", tag, data);
    });
}

pub fn show<T: TagDataRepository>(repo: T, tag: String) {
    match repo.get_tag_data(&tag) {
        Some(cmd) => println!("{}: {}", tag, cmd),
        None => println!("Command not found"),
    }
}
