use crate::repo::tag_data_repository::TagDataRepository;

use super::ALL_SUBCOMMAND;
use anyhow::Result;

pub fn add<T: TagDataRepository>(tag: String, command: String, mut repo: T) -> Result<()> {
    println!("Adding tag {} with command {}", tag, command);
    if tag == ALL_SUBCOMMAND {
        println!("tag {} is reserved.", ALL_SUBCOMMAND);
        return Err(anyhow::anyhow!("tag is reserved."));
    }
    repo.add_tag_data(tag, command);
    Ok(())
}
