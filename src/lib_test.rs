#[test]
fn app_new() {
    let res = crate::index_generator();
    assert!(res.is_ok());
}
