use std::path;

use anyhow::Result;
use clap::Parser;
use directories::BaseDirs;
use repo::{hashmap_repository::HashMapRepository, tag_data_repository::TagDataRepository};

mod cmd;
mod repo;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,

    #[clap(long)]
    data_path: Option<String>,
}

#[derive(Parser)]
enum Command {
    Add(Add),
    Delete(Delete),
    Show(Show),
    Search(Search),
}

#[derive(Parser)]
struct Add {
    tag: String,
    command: String,
}

#[derive(Parser)]
struct Delete {
    tag: String,
}

#[derive(Parser)]
struct Show {
    target: String,
    tag: Option<String>,
}

#[derive(Parser)]
struct Search {
    search_str: Option<String>,
}

const COMMAND_NAME: &str = "tagcm";
const DEFAULT_FILE_NAME: &str = "tags.json";

fn get_data_path(data_path: Option<String>) -> String {
    if let Some(path) = data_path {
        return path;
    }
    if let Some(base_dir) = BaseDirs::new() {
        return path::Path::new(base_dir.config_dir().to_str().unwrap())
            .join(COMMAND_NAME)
            .join(DEFAULT_FILE_NAME)
            .into_os_string()
            .into_string()
            .unwrap();
    }
    String::new()
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let mut repo: HashMapRepository = repo::hashmap_repository::HashMapRepository::new();
    repo.init(&get_data_path(opts.data_path))?;

    match opts.command {
        Command::Add(opt) => {
            cmd::add::add(opt.tag, opt.command, repo)?;
        }
        Command::Delete(opt) => {
            cmd::delete::delete(repo, opt.tag);
        }
        Command::Show(opt) => {
            if opt.target == "all" {
                cmd::show::show_all(repo);
            } else {
                cmd::show::show(repo, opt.target);
            }
        }
        Command::Search(opt) => match opt.search_str {
            Some(_) => {
                let tags = cmd::search::search(&repo, opt.search_str.unwrap())?;
                for tag in tags {
                    println!("tag: {}, command: {}", tag.tag, tag.command);
                }
            }
            None => {
                cmd::search::search_by_input(repo)?;
            }
        },
    }
    Ok(())
}
