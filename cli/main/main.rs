use std::{path::PathBuf, fs};
use brapi_client::client::Client;
use brapi_cli_live::Request as LiveRequest;

/// Berylsoft brapi CLI
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
    let access = fs::read_to_string(access_path).unwrap();
    let client = Client::with_access(access, None).unwrap();
    match inner {
        Request::Live(LiveArgs { inner }) => inner.call(client).await,
    }
}
