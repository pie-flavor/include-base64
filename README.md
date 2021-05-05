# `include-base64`

A macro to include a file as a base-64 encoded string literal at compile time.

```rust
const MY_FILE_BUT_IN_BASE64: &str = include_base64!("my_file.txt");
```