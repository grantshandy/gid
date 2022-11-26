use argh::FromArgs;
use google_tasks1::{
    api::{Task, TaskMethods},
    hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector,
    Result,
};

/// add a task
#[derive(FromArgs)]
#[argh(subcommand, name = "add")]
pub struct Add {
    /// names or index of the task to remove
    #[argh(positional)]
    names: Vec<String>,
}

pub async fn add_task<'a>(
    add: Add,
    list_id: String,
    methods: TaskMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    for name in add.names {
        let task = Task {
            title: Some(name),
            ..Default::default()
        };

        methods.insert(task, &list_id).doit().await?;
    }

    Ok(())
}
