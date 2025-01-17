# hop

Terminal chat client written in Rust

## Why

This project has no real world use: it's just for me to learn rust, async/await, network programming.

## How it works

- Server runs
- Clients log into their accounts or sign up
- Server saves the user into a `UserConn`
- Which then gets put into a `HashMap<IpAddr, UserConn>`
- Server listens, asynchronously, to `Request`s
    - Match on `Request` and execute
        - Deserialize args with rmp_serde
- Client listens to server too, which sends data too
    - eg when a message is sent, 
        - everyone in that server that's viewing that channel will get the whole message
        - everyone in that server but not viewing will get a "header" for a message (no content)
