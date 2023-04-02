# capechoserver
### simple echo tcp server and cli client using `tokio`

### Feat
1. handles multiple clients. no reconnect atm
2. has common library for message handling
3. only some tests ((
4. tested on Ubuntu 20.04 lts, but it is expected to work on other machine types

### RUN
1. Build it with `cargo build` or `cargo build --release`
2. run server either with `cargo run --bin server` or, in case of production release, its binary in `./target/release/server`
3. open another terminal and run client `cargo run --bin client` or related binary
