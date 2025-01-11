# tokenizer-rs

A simple tokenizer library that can be leveraged for various purposes.


## Usage

```rust
use tokenizer_rs::*;

let contents = "let x = 1;";
let location: PathBuf = "test.rs".into();
let tokens = tokenize(contents, Location::from(location)).unwrap();
```
