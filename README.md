# Full Rust AI Chat

This is an implentation of an AI Chat as shown by Code to the Moon youtube channel.

As it contains the language model, make sure to have git-lfs installed before cloning the repository:
```shell
git lfs install
git clone git@github.com:kantum/ai-chat.git
cd ai-chat
```

You will need the cargo leptos cli to build the app:
```shell
cargo install --locked cargo-leptos
```

## Running the project for dev

```shell
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

```shell
cargo leptos watch
```

## Compiling for Release
```shell
cargo leptos build --release
```
Will generate your server binary in target/server/release and your site package in target/site

## Run the Release Binary
```shell
./target/server/release/ai-chat
```

## Testing Your Project
```shell
cargo leptos end-to-end
```

```shell
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
ai-chat
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="ai-chat"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.
