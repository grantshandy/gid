use argh::FromArgs;
use google_tasks1::{
    api::{TaskList, TasklistMethods},
    hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector,
    Result,
};

/// create a task list
#[derive(FromArgs)]
#[argh(subcommand, name = "add")]
pub struct Add {
    /// the name of the task list
    #[argh(positional)]
    name: String,
}

pub async fn add_list<'a>(
    add: Add,
    methods: TasklistMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let list = TaskList {
        title: Some(add.name),
        ..Default::default()
    };
    
    methods.insert(list).doit().await?;

    Ok(())
}
