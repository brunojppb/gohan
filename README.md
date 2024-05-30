<p align="center"><br><img src="./icon.png" width="128" height="128" alt="Gohan icon" /></p>
<h2 align="center">Gohan</h2>
<p align="center">
  Static site generator based on Markdown files
</p>

## Features

This is a learning project I aim to use to generate my own websites. I am going to slowly work on the following features:

- Markdown Lexer and Parser
- Markdown to HTML compiler
- WebAssembly Web app to test the parser/compiler
- Syntax highlighting for the most common languages I use
- Support for templating

### Try it yourself

I've setup continuous deployment for a Web app built from the [web_repl](./web_repl/README.md) crate.

Try it yourself here: [gohan.bpaulino.com](https://gohan.bpaulino.com)

> [!CAUTION]
> This is a highly experimental project and the markdown parser does not handle all cases just yet.
> I aim to slowly implement the [CommonMark spec](https://commonmark.org/), but it's way too far away from being anywhere completed. Do not use this in production.
