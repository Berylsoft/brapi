#[tokio::main]
async fn main() {
    let access_path = std::env::var_os("BAPI_ACCESS_PATH").unwrap();
    let access = std::fs::read_to_string(access_path).unwrap();
    let client = brapi::client::Client::with_access(access, None).unwrap();
    client.update_wbi_key().await.unwrap();
    let res = client.call(&brapi::live::feed::GetHostsInfo { roomid: 5440 }).await.unwrap();
    let _ = dbg!(res);
}
