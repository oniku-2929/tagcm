use clap::Parser;

mod cmd;

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
struct Search {
    search_str: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Add(opt) => {
            cmd::add::add(opt.tag, opt.command);
        }
        Command::Delete(opt) => {
            cmd::delete::delete(opt.tag);
        }
        Command::Show => {
            cmd::show::show("aa".to_string());
        }
        Command::Search(opt) => {
            cmd::search::search(opt.search_str);
        }
    }
}
