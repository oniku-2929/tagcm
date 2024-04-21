use crate::repo::tag_data_repository::TagDataRepository;

use super::ALL_SUBCOMMAND;
use anyhow::Result;

pub fn add<T: TagDataRepository>(tag: String, command: String, repo: &mut T) -> Result<()> {
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
    use crate::repo::unittest_repository::UnitTestRepository;
    #[test]
    fn test_add() {
        let mut repo = UnitTestRepository::new();
        add("test".to_string(), "echo add test".to_string(), &mut repo).unwrap();
        assert_eq!(repo.get_tag_data("test").unwrap(), "echo test");
    }
}
