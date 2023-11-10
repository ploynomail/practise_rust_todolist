use clap::{Args, Parser, Subcommand};
#[derive(Debug, Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap[about = "todo info"]]
    Info,
    #[clap(about = "Add a todo item.")]
    Add {
        #[clap(help = "The item content to add.", required = true)]
        content: Option<String>,
    },

    Get {
        #[clap(help = "The item id to get.")]
        id: Option<i32>,
    },
    #[clap(about = "Remove a todo item.")]
    #[clap(visible_aliases = & ["rm"])]
    Remove {
        #[clap(help = "The item id to remove.")]
        id: Option<i32>,
    },
    #[clap(about = "List all the todo items.")]
    #[clap(visible_aliases = & ["ls", "ll", "la"])]
    List,
}
