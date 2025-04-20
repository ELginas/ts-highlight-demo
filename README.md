# ts-highlight-demo

tree-sitter syntax highlighting demo with [my `tree-sitter-json`](https://github.com/ELginas/tree-sitter-json) package.

## Testing

Clone [my `tree-sitter-json`](https://github.com/ELginas/tree-sitter-json) to have this folder structure:

```
├── tree-sitter-json
└── ts-highlight-demo
```

And then run the demo inside `ts-highlight-demo` directory:

```sh
cargo r -- ../tree-sitter-json/example_file
```
