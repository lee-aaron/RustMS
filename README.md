# RustMS 🍁

RustMS is an attempt at implementing the Maplestory server end from scratch in Rust. 

This is a fork of the original owner's work. I intend to 
try updating and making some changes too.

The aim is to be compatible with HeavenMS and it's basic localhost client.

Next would be writing a client in Rust.

## Setup

```
docker-compose up

# https://diesel.rs/guides/getting-started
cargo install diesel_cli
diesel migration list
diesel migration run
```