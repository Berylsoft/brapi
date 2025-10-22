async fn _main() {
    let client = brapi::client::Client::bare();
    let res = client.call(&brapi::live::info::GetRoomInfo { sroomid: 953650 }).await;
    let _ = dbg!(res);
}

fn main() {
    async_global_executor::block_on(_main())
}
