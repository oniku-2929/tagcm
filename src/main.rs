use std::path;

use anyhow::Result;
use clap::Parser;
use repo::{hashmap_repository::HashMapRepository, tag_data_repository::TagDataRepository};

mod cmd;
mod repo;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
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

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let mut repo: HashMapRepository = repo::hashmap_repository::HashMapRepository::new();
    let path = path::Path::new("data.json");
    match repo.init(path.to_str().unwrap()) {
        Ok(_) => {}
        Err(e) => {
            println!("file: {} does not exist. {}", path.to_str().unwrap(), e);
        }
    }

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
