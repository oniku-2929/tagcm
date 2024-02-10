use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Add(Add),
    Delete(Delete),
    Show,
    Search,
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

fn main() {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Add(add) => {
            println!("Adding tag {} with command {}", add.tag, add.command);
        }
        Command::Delete(delete) => {
            println!("Deleting tag {}", delete.tag);
        }
        Command::Show => {
            println!("Showing tags");
        }
        Command::Search => {
            println!("Searching for a tag");
        }
    }
}
