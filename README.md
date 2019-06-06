# Shadow Clone
A macro to clone variables into the current scope shadowing old ones.

[![pipeline status](https://gitlab.com/efunb/shadow-clone/badges/stable/pipeline.svg)](https://gitlab.com/efunb/shadow-clone/commits/stable)
[![Latest version](https://img.shields.io/crates/v/shadow-clone.svg)](https://crates.io/crates/shadow-clone)
[![Latest Docs](https://docs.rs/shadow-clone/badge.svg)](https://docs.rs/shadow-clone)
[![downloads-badge](https://img.shields.io/crates/d/shadow-clone.svg)](https://crates.io/crates/shadow-clone)

## Help

If you run into any issues or need help with using `shadow-clone` in your project please email [incoming+efunb-shadow-clone-12722979-issue-@incoming.gitlab.com](mailto:incoming+efunb-shadow-clone-12722979-issue-@incoming.gitlab.com).

## How to use

Add 
```toml
shadow-clone = "0.1"
```
to your `cargo.toml` under `[dependencies]` and add
```rust
use shadow_clone::shadow_clone;
```
to your main file.

## Examples
```rust,compile_fail
let s = "foo".to_string();
let c = move |x: i32| format!("{}{}", s, x);
let bar = s;
```
This will not compile as `s` has been moved into the closure.

This issue can be solved with this macro.
```rust
use shadow_clone::shadow_clone;
let s = "foo".to_string();
{
    shadow_clone!(s);
    let c = move |x: i32| format!("{}{}", s, x);
}
let bar = s;
```
That expands to,
```rust
use shadow_clone::shadow_clone;
let s = "foo".to_string();
{
    let s = s.clone();
    let c = move |x: i32| format!("{}{}", s, x);
}
let bar = s;
```
You can also clone multiple variables separated by commas. `shadow_clone!(foo, bar);`

## Docs

[API Documentation](https://docs.rs/shadow-clone)

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/shadow-clone).**