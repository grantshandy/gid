use argh::FromArgs;
use google_tasks1::{
    api::TasklistMethods, hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Result,
};

/// remove a task list
#[derive(FromArgs)]
#[argh(subcommand, name = "remove")]
pub struct Remove {
    /// the name or index of the task list
    #[argh(positional)]
    name: String,
}

pub async fn remove_list<'a>(
    remove: Remove,
    methods: TasklistMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let id = crate::get_list_id_from_name(&remove.name, &methods).await?;

    methods.delete(&id).doit().await?;

    Ok(())
}
