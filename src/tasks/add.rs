use argh::FromArgs;
use google_tasks1::{Result, api::{TaskMethods, Task}, hyper_rustls::HttpsConnector, hyper::client::HttpConnector};


/// add a task
#[derive(FromArgs)]
#[argh(subcommand, name ="add")]
pub struct Add {
    /// name or index of the task to add
    #[argh(positional)]
    name: String,
}

pub async fn add_task<'a>(add: Add, list_id: String, methods: TaskMethods<'a, HttpsConnector<HttpConnector>>) -> Result<()> {
    let task = Task {
        title: Some(add.name),
        ..Default::default()
    };
    
    methods.insert(task, &list_id).doit().await?;
    
    Ok(())
}