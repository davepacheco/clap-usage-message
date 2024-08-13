use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[clap(env = "MY_ARG", global = true)]
    my_arg: Option<String>,

    #[command(subcommand)]
    command: MyCommands,
}

#[derive(Debug, Clone, Subcommand)]
enum MyCommands {
    One(OneSubcommands),
}

#[derive(Debug, Clone, Args)]
struct OneSubcommands {
    #[command(subcommand)]
    command: MySubcommand,
}

#[derive(Debug, Clone, Subcommand)]
enum MySubcommand {
    Two,
}

fn main() {
    let cli = Cli::parse();
    eprintln!("{:?}", cli);
}
