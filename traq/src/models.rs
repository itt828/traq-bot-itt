pub mod api;
pub mod event;
#[macro_export]
macro_rules! model_test {
    ($f:ident,$t:ty,$s:expr) => {
        #[test]
        fn $f() {
            let data = $s;
            let r = serde_json::from_str::<$t>(data);
            assert!(r.is_ok());
        }
    };
}
