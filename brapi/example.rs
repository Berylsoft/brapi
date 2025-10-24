#[tokio::main]
async fn main() {
    let client = brapi::client::Client::bare();
    let res = client.call(&brapi::wbi_api::GetBasicInfo).await;
    let _ = dbg!(res);
}
