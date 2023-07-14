# BRAPI

The second Rust implementation on GitHub of third-party REST API client for a website. Designed to be lightweight and efficient.

It's formerly `livekit-api` ([LiveKit](https://github.com/Berylsoft/LiveKit)), so there are only a few APIs related to live business. More APIs will be implemented later. PRs are also welcomed.

- Abstract the API to the trait `RestApi`, bringing both clear readable code and reduced runtime overhead. And you are able to implement your own API!

- Directly based on `hyper`, reducing the redundancy of `reqwest` which brings more binary size.

- Parse `Access` from cookies for requests that require login.

```rust
use brapi::{client::Client, live::info::GetRoomInfo};
let client = Client::new_bare();
let room_info = client.call(&GetRoomInfo { sroomid: 23590843 }).await.unwrap();
assert_eq!(room_info.uid, 573732342);
println!("{:?}", room_info);
```
