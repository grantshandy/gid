use std::{env, fs, path::PathBuf, process};

use google_tasks1 as gtasks;

use argh::FromArgs;
use colored::Colorize;
use gtasks::{
    api::{TaskMethods, TasklistMethods},
    hyper::{client::HttpConnector, Client},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    oauth2::{
        self, authenticator::Authenticator, InstalledFlowAuthenticator, InstalledFlowReturnMethod,
    },
    Error, Result, TasksHub,
};
use serde::Deserialize;
use serde_json::json;
use tabled::{Style, Table, Tabled};

mod lists;
mod tasks;

use lists::Lists;
use tasks::Tasks;

const TOKEN_FILENAME: &str = ".gid-auth.json";
const CONFIG_FILENAME: &str = "gid.toml";

const AUTH_SECRETS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/clientsecret"));
const DEFAULT_USER_AGENT: &str = concat!("gid/", env!("CARGO_PKG_VERSION"));

/// Use Google Tasks from the command line.
#[derive(FromArgs)]
struct Args {
    #[argh(
        option,
        short = 'b',
        description = "set the base url to use in all requests to the server"
    )]
    base_url: Option<String>,
    #[argh(
        option,
        short = 'a',
        description = "set the user-agent header field to use in all requests to the server",
        default = "DEFAULT_USER_AGENT.to_string()"
    )]
    user_agent: String,
    #[argh(
        option,
        short = 'c',
        description = "custom path to a config file",
        default = "config()"
    )]
    config_path: PathBuf,
    #[argh(subcommand)]
    nested: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Lists(Lists),
    Task(Tasks),
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    let auth = get_auth().await;
    let mut hub = TasksHub::new(
        Client::builder().build(
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    // config
    let config: Config = match fs::read(args.config_path) {
        Ok(bytes) => match toml::from_slice::<Config>(&bytes) {
            Ok(config) => config,
            Err(_err) => {
                print_error("malformed config path, using default");
                Config::default()
            }
        },
        Err(_err) => Config::default(),
    };

    // set client options
    hub.user_agent(args.user_agent);
    if let Some(base_url) = args.base_url {
        hub.base_url(base_url);
    }

    let err = match args.nested {
        SubCommand::Lists(options) => lists::manage(config, options, hub).await,
        SubCommand::Task(options) => tasks::manage(config, options, hub).await,
    };

    if let Err(err) = err {
        print_error(&err.to_string());
        process::exit(1);
    }
}

async fn get_auth() -> Authenticator<HttpsConnector<HttpConnector>> {
    let secret = oauth2::parse_application_secret(
        base64::decode(AUTH_SECRETS).expect("clientsecret not base64"),
    )
    .expect("malformed clientsecret format");

    let mut token_file = dirs::config_dir().unwrap_or(PathBuf::from("~/"));
    token_file.push(TOKEN_FILENAME);

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(token_file)
        .build()
        .await
        .expect("failed to authenticate");

    return auth;
}

fn config() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("~/"));

    path.push(CONFIG_FILENAME);

    path
}

fn print_error(msg: &str) {
    println!("{}: {}", "Error".bold().red(), msg);
}

async fn get_list_id_from_name<'a>(
    query: &str,
    methods: &TasklistMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<String> {
    match methods
        .list()
        .doit()
        .await?
        .1
        .items
        .unwrap_or_default()
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            let name = item.title.clone().unwrap_or_default();

            if query == name || query.parse::<usize>() == Ok(idx) {
                item.id.clone()
            } else {
                None
            }
        })
        .nth(0)
    {
        Some(id) => Ok(id),
        None => Err(Error::BadRequest(json!("Task name or index not found."))),
    }
}

pub async fn get_task_id_from_name<'a>(
    list_id: &str,
    query: &str,
    methods: &TaskMethods<'a, HttpsConnector<HttpConnector>>,
) -> Result<Option<String>> {
    match methods
        .list(list_id)
        .doit()
        .await?
        .1
        .items
        .unwrap_or_default()
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            let name = item.title.clone().unwrap_or_default();

            if query == name || query.parse::<usize>() == Ok(idx) {
                item.id.clone()
            } else {
                None
            }
        })
        .nth(0)
    {
        Some(id) => Ok(Some(id)),
        None => Ok(None),
    }
}

pub fn get_styled_table<T: Tabled, I: IntoIterator<Item = T>>(style: &str, data: I) -> String {
    let mut table = Table::new(data);

    match style {
        "markdown" => table.with(Style::markdown()),
        "empty" => table.with(Style::empty()),
        "blank" => table.with(Style::blank()),
        "ascii" => table.with(Style::ascii()),
        "ascii_rounded" => table.with(Style::ascii_rounded()),
        "modern" => table.with(Style::modern()),
        "sharp" => table.with(Style::sharp()),
        "rounded" => table.with(Style::rounded()),
        "extended" => table.with(Style::extended()),
        "dots" => table.with(Style::dots()),
        _ => table.with(Style::rounded()),
    };

    table.to_string()
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_list: String,
    #[serde(default)]
    pub table_style: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_list: "0".to_string(),
            table_style: "rounded".to_string(),
        }
    }
}
