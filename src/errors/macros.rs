#[macro_export]
macro_rules! dctx {
    () => {{
        $crate::errors::DCtx::new(format!("{}:{}:{}", file!(), line!(), column!()))
    }};
}
