use shadow_clone::shadow_clone;

#[test]
fn mutable_clone() {
    let s = "foo".to_string();
    {
        shadow_clone!(mut s);
        let _ = move |_: i32| s = "changed".to_string();
    }
}
#[test]
#[allow(unused_mut)]
fn random_silly_clones() {
    let s1 = "foo".to_string();
    let s2 = "bar".to_string();
    let s3 = "baz".to_string();
    {
        shadow_clone!(mut s1, (mut) s2, s3);
        let _ = move |_: i32| s1 = format!("{}{}{}", s1, s2, s3);
    }
}
