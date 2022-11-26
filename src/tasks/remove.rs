use argh::FromArgs;
use google_tasks1::{
    api::TaskMethods, hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Error, Result,
};
use serde_json::json;

/// remove a task
#[derive(FromArgs)]
#[argh(subcommand, name = "remove")]
pub struct Remove {
    /// names or index of the task to remove
    #[argh(positional)]
    names: Vec<String>,
}

pub async fn remove_task<'a>(
    options: Remove,
    list_id: String,
    methods: TaskMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let mut task_ids: Vec<String> = Vec::new();

    for name in &options.names {
        let task_id = match crate::get_task_id_from_name(&list_id, name, &methods).await? {
            Some(task_id) => task_id,
            None => return Err(Error::BadRequest(json!("Bad task name"))),
        };

        task_ids.push(task_id);
    }

    for task_id in task_ids {
        methods.delete(&list_id, &task_id).doit().await?;
    }

    Ok(())
}
