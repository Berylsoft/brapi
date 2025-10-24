#[tokio::main]
async fn main() {
    let access_path = std::env::var_os("BAPI_ACCESS_PATH").unwrap();
    let access = std::fs::read_to_string(access_path).unwrap();
    let client = brapi::client::Client::with_access(access, None).unwrap();
    let res = client.call(&brapi::wbi_api::GetBasicInfo).await;
    let _ = dbg!(res);
}
