//! Go the the [readme](https://crates.io/crates/shadow-clone) file for documentation.

#![deny(clippy::pedantic, missing_docs)]

/// Use this macro to clone variables into the current scope shadowing old ones.
#[macro_export]
macro_rules! shadow_clone {
    ($ ($to_clone:ident) ,*) => {
        $(
            let $to_clone = $to_clone.clone();
        )*
    };
}

// fn main() {
//     let s = "foo".to_string();
//     let c = move |x: i32| format!("{}{}", s, x);
//     let bar = s;
// }