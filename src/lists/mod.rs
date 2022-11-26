use argh::FromArgs;
use google_tasks1::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Result, TasksHub};

mod add;
mod remove;
mod show;

use add::Add;
use remove::Remove;
use show::Show;

use crate::Config;

/// manage task lists
#[derive(FromArgs)]
#[argh(subcommand, name = "lists")]
pub struct Lists {
    #[argh(subcommand)]
    nested: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Create(Add),
    Delete(Remove),
    Show(Show),
}

pub async fn manage(config: Config, options: Lists, hub: TasksHub<HttpsConnector<HttpConnector>>) -> Result<()> {
    let methods = hub.tasklists();

    match options.nested {
        SubCommand::Create(options) => add::add_list(options, methods).await,
        SubCommand::Delete(options) => remove::remove_list(options, methods).await,
        SubCommand::Show(options) => show::show_list(options, config, methods).await,
    }
}
