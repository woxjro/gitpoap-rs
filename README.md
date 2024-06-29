# gitpoap-rs
A Rust implementation of the [gitpoap](https://www.gitpoap.io/).
It supports both the `wasm` and `native` targets. So, you can use it in the browser or in the terminal.

## Prerequisites
- `wasm-pack`
    - To install `wasm-pack`, follow the instructions [here](https://rustwasm.github.io/wasm-pack/installer/).

## Build
Compile to `native`:

```sh
$ cargo build
```

Compile to `wasm`:

```sh
$ wasm-pack build --target web
```

## Usage
```rust
use gitpoap_rs::v1::get_gitpoaps_for_github_user;

#[tokio::main]
async fn main() {
    let github_handle = "woxjro";
    let response = get_gitpoaps_for_github_user(github_handle, None).await;
    match response {
        Ok(gitpoaps_response) => {
            dbg!("{:?}", gitpoaps_response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```
