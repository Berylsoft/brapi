#[tokio::main]
async fn main() {
    let client = brapi::client::Client::bare();
    let res = client.call(&brapi::live::info::GetRoomInfo { sroomid: 953650 }).await;
    let _ = dbg!(res);
}
