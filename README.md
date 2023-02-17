# normalize-url-rs

normalize-url-rs is a port of Node.js [normalize-url](https://github.com/sindresorhus/normalize-url) package
for the [Rust](http://rust-lang.org/) programming language.
    
Documentation: https://docs.rs/normalize-url-rs

## Sample usage
```rs
use normalize_url_rs::{normalize_url, OptionsBuilder};

let options = OptionsBuilder::default().build().unwrap();
let result = normalize_url("https://www.rust-lang.org/", options);

assert_eq!(result.unwrap(), "https://rust-lang.org")
```

## Known differences vs original Node.js library

- Custom protocols are not supported
- Data URLs are not supported
