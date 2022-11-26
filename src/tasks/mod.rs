use argh::FromArgs;
use chrono::Utc;
use google_tasks1::{
    api::{Task as ApiTask, TaskMethods},
    hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector,
    Result, TasksHub,
};
use tabled::Tabled;

use crate::{get_styled_table, get_task_id_from_name, Config};

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
    Finish(Finish),
}

pub async fn manage(
    config: Config,
    options: Tasks,
    hub: TasksHub<HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let methods = hub.tasks();
    let list_id = crate::get_list_id_from_name(
        &options.list.unwrap_or(config.default_list.clone()),
        &hub.tasklists(),
    )
    .await?;

    match options.nested {
        SubCommand::List(options) => list_tasks(options, config, list_id, methods).await?,
        SubCommand::Add(options) => add_task(options, list_id, methods).await?,
        SubCommand::Remove(options) => remove_task(options, list_id, methods).await?,
        SubCommand::Finish(options) => finish_task(options, list_id, methods).await?,
    };

    Ok(())
}

/// Show tasks.
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
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
        .map(|(idx, m)| Task {
            id: idx.to_string(),
            title: m.title.clone().unwrap_or_default(),
        })
        .collect();

    println!("{}", get_styled_table(&config.table_style, tasks));

    Ok(())
}

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
        let task = ApiTask {
            title: Some(name),
            ..Default::default()
        };

        methods.insert(task, &list_id).doit().await?;
    }

    Ok(())
}

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
        let task_id = crate::get_task_id_from_name(&list_id, name, &methods).await?;

        task_ids.push(task_id);
    }

    for task_id in task_ids {
        methods.delete(&list_id, &task_id).doit().await?;
    }

    Ok(())
}

/// finish a task
#[derive(FromArgs)]
#[argh(subcommand, name = "finish")]
pub struct Finish {
    /// names or indexes of the tasks
    #[argh(positional)]
    names: Vec<String>,
}

pub async fn finish_task<'a>(
    options: Finish,
    list_id: String,
    methods: TaskMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<()> {
    let mut tasks = Vec::new();

    for name in &options.names {
        let task_id = get_task_id_from_name(&list_id, name, &methods).await?;
        let (_resp, task) = methods.get(&list_id, &task_id).doit().await?;

        tasks.push(task);
    }

    for mut task in tasks {
        println!("{:?}", task);
        
        task.completed = Some(Utc::now().to_rfc3339());
        task.hidden = Some(true);
        
        println!("{:?}", task);

        if let Some(id) = task.id.clone() {
            methods.patch(task, &list_id, &id).doit().await?;
        }
    }

    Ok(())
}
