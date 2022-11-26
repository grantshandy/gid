use argh::FromArgs;
use google_tasks1::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Result, TasksHub};

mod list;
mod add;
mod remove;

use list::List;
use add::Add;
use remove::Remove;

use crate::Config;

/// manage tasks
#[derive(FromArgs)]
#[argh(subcommand, name = "tasks")]
pub struct Tasks {
    /// which list to edit
    #[argh(option, short = 'l')]
    list: Option<String>,
    #[argh(subcommand)]
    nested: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    List(List),
    Add(Add),
    Remove(Remove),
}

pub async fn manage(config: Config, options: Tasks, hub: TasksHub<HttpsConnector<HttpConnector>>) -> Result<()> {
    let methods = hub.tasks();
    let list_id = crate::get_list_id_from_name(&options.list.unwrap_or(config.default_list), &hub.tasklists()).await?;

    match options.nested {
        SubCommand::List(options) => list::list_tasks(options, list_id, methods).await?,
        SubCommand::Add(options) => add::add_task(options, list_id, methods).await?,
        SubCommand::Remove(options) => remove::remove_task(options, list_id, methods).await?,
    };

    Ok(())
}
