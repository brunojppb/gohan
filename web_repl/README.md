## Web REPL

Web application for converting Markdown to HTML in real-time.

### Development

This crate uses [yew](https://yew.rs/) to generate a Web app based on [WebAssembly](https://webassembly.org/) that can be deployed to any static site hosting.

First, add WebAssembly as a compilation target with:

```shell
rustup target add wasm32-unknown-unknown
```

Now install [Trunk](https://trunkrs.dev/) with:

```shell
cargo install --locked trunk
```

Now start the dev server with:

```shell
trunk serve --open
```
