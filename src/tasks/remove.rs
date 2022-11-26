use argh::FromArgs;
use google_tasks1::{
    api::TaskMethods,
    hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector,
    Result, Error,
};
use serde_json::json;

/// remove a task
#[derive(FromArgs)]
#[argh(subcommand, name = "remove")]
pub struct Remove {
    /// name or index of the task to remove
    #[argh(positional)]
    name: String,
}

pub async fn remove_task<'a>(
    options: Remove,
    list_id: String,
    methods: TaskMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let task_id = match
        crate::get_task_id_from_name(&list_id, &options.name, &methods).await? {
            Some(task_id) => task_id,
            None => return Err(Error::BadRequest(json!("Bad task name"))),
        };

    methods.delete(&list_id, &task_id).doit().await?;

    Ok(())
}
