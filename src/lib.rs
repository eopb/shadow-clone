//! Go the the [readme](https://crates.io/crates/shadow-clone) file for more documentation.

#![deny(clippy::pedantic, missing_docs)]

/// Use this macro to clone variables into the current scope shadowing old ones.
///
/// # Examples
/// ```rust,compile_fail
/// let s = "foo".to_string();
/// let c = move |x: i32| format!("{}{}", s, x);
/// let bar = s;
/// ```
/// This will not compile as `s` has been moved into the closure.
///
/// This issue can be solved with this macro.
/// ```rust
/// use shadow_clone::shadow_clone;
/// let s = "foo".to_string();
/// {
///     shadow_clone!(s);
///     let c = move |x: i32| format!("{}{}", s, x);
/// }
/// let bar = s;
/// ```
/// You can also clone multiple variables separated by commas. `shadow_clone!(foo, bar);`
#[macro_export]
macro_rules! shadow_clone {
    ($ ($to_clone:ident) ,*) => {
        $(
            let $to_clone = $to_clone.clone();
        )*
    };
}
