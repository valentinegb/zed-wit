# Zed WIT

WIT support for Zed.

Features currently include:

- Syntax highlighting
- Basic Zed language features
- LSP, provided by [wit-lsp](https://github.com/Michael-F-Bryan/wit-lsp)
- Completion labels

## Requirements

In order for all Language Server features to work, you will need to make sure yuou have the corresponding `wit-cli` tool installed in your path:

```sh
cargo install --git https://github.com/Michael-F-Bryan/wit-lsp.git wit-cli --locked
```
