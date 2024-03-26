use crate::repo::tag_data_repository::TagDataRepository;

use super::ALL_SUBCOMMAND;
use anyhow::Result;

pub fn add<T: TagDataRepository>(tag: String, command: String, mut repo: &T) -> Result<()> {
    if tag == ALL_SUBCOMMAND {
        println!("tag {} is reserved.", ALL_SUBCOMMAND);
        return Err(anyhow::anyhow!("tag is reserved."));
    }
    repo.add_tag_data(tag, command);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::hashmap_repository::HashMapRepository;
    #[test]
    fn test_add() {
        let repo = HashMapRepository::new();
        add("test".to_string(), "echo test".to_string(), &repo).unwrap();
        assert_eq!(repo.get_tag_data("test").unwrap(), "echo test");
    }
}
