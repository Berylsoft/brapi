use std::{path::PathBuf, fs};
use bilibili_restapi_client::{client::Client, access::Access};
use bilibili_restapi_cli_live::Request as LiveRequest;

/// Berylsoft bilibili-restapi CLI
#[derive(argh::FromArgs)]
struct Args {
    /// json access file path
    #[argh(option, short = 'a')]
    access_path: Option<PathBuf>,
    #[argh(subcommand)]
    inner: Request,
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum Request {
    Live(LiveArgs),
}

/// live interact
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "live")]
struct LiveArgs {
    #[argh(subcommand)]
    inner: LiveRequest,
}


#[tokio::main]
async fn main() {
    let Args { access_path, inner } = argh::from_env();
    let access_path = access_path.unwrap_or_else(|| std::env::var_os("BAPI_ACCESS_PATH").unwrap().into());
    let access: Access = serde_json::from_reader(fs::OpenOptions::new().read(true).open(access_path).unwrap()).unwrap();
    let client = Client::new(Some(access), None);
    match inner {
        Request::Live(LiveArgs { inner }) => inner.call(client).await,
    }
}
