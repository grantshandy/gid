use std::{env, path::PathBuf};

use google_tasks1 as tasks;

use argh::FromArgs;
use tasks::{
    hyper::{client::HttpConnector, Client},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    oauth2::{
        self, authenticator::Authenticator, InstalledFlowAuthenticator, InstalledFlowReturnMethod,
    },
    TasksHub,
};

const TOKEN_FILE: &str = ".gid-auth.json";
const AUTH_SECRET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/clientsecret"));
// const SCOPES: &[&str; 2] = &[
//     "https://www.googleapis.com/auth/tasks",
//     "https://www.googleapis.com/auth/tasks.readonly",
// ];
const USER_AGENT: &str = concat!("gid/", env!("CARGO_PKG_VERSION"));

/// Use Google Tasks from the command line.
#[derive(FromArgs)]
struct Args {}

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
    hub.user_agent(USER_AGENT.to_string());
    
    let (_resp, tasklists) = hub.tasklists().list().doit().await.expect("failed to download");

    println!("Your Tasks:");
    for task in tasklists.items.unwrap() {
        println!("{}", task.title.unwrap());
    }
}

async fn get_auth() -> Authenticator<HttpsConnector<HttpConnector>> {
    let secret = oauth2::parse_application_secret(
        base64::decode(AUTH_SECRET).expect("clientsecret not base64"),
    )
    .expect("malformed clientsecret format");

    let mut token_file = dirs::config_dir().unwrap_or(PathBuf::from("~/"));
    token_file.push(TOKEN_FILE);

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(token_file)
        .build()
        .await
        .expect("failed to authenticate");

    return auth;
}
