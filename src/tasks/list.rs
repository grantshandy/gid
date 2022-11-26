use argh::FromArgs;
use google_tasks1::{
    api::TaskMethods,
    hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector,
    Result,
};
use tabled::Tabled;

use crate::{get_styled_table, Config};

/// Show tasks.
#[derive(FromArgs)]
#[argh(subcommand, name = "show")]
pub struct List {}

#[derive(Tabled)]
struct Task {
    #[tabled(rename = "#")]
    id: String,
    #[tabled(rename = "Name")]
    title: String,
}

pub async fn list_tasks<'a>(
    _show: List,
    config: Config,
    list_id: String,
    methods: TaskMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let (_resp, tasks) = methods.list(&list_id).doit().await?;
    
    let tasks: Vec<Task> = tasks
        .items
        .unwrap_or_default()
        .iter()
        .enumerate()
        .map(|(idx, m)| {
            Task {
                id: idx.to_string(),
                title: m.title.clone().unwrap_or_default(),
            }
        })
        .collect();
    
    println!("{}", get_styled_table(&config.table_style, tasks));

    Ok(())
}
