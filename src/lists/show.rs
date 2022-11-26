use argh::FromArgs;
use google_tasks1::{
    api::TasklistMethods, hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Result,
};
use tabled::Tabled;

use crate::{get_styled_table, Config};

/// list all task lists
#[derive(FromArgs)]
#[argh(subcommand, name = "show")]
pub struct Show {
    /// maximum number of task lists returned on one page
    #[argh(option, short = 'm')]
    max_results: Option<i32>,
}

#[derive(Tabled)]
struct TaskList {
    #[tabled(rename = "#")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
}

pub async fn show_list<'a>(
    show: Show,
    config: Config,
    methods: TasklistMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let result = methods.list();

    let result = if let Some(max_results) = show.max_results {
        result.max_results(max_results)
    } else {
        result
    };

    let (_resp, tasks) = result.doit().await?;

    let items: Vec<TaskList> = tasks
        .items
        .unwrap_or_default()
        .iter()
        .enumerate()
        .map(|(idx, l)| TaskList {
            name: l.title.clone().unwrap_or_default(),
            id: idx.to_string(),
        })
        .collect();

    println!("{}", get_styled_table(&config.table_style, items));

    Ok(())
}
