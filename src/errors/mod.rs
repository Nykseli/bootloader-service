mod macros;

/// Error context that should be created with `dctx!()` macro
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DCtx(String);

impl DCtx {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum DErrorType {
    GrubParse(String),
    Io(String, std::io::Error),
    Sqlx(String, sqlx::Error),
    Zbus(String, zbus::Error),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DError {
    ctx: DCtx,
    error: DErrorType,
}

impl DError {
    fn new(ctx: DCtx, error: DErrorType) -> Self {
        Self { ctx, error }
    }

    pub fn grub_parse_error<M: Into<String>>(ctx: DCtx, message: M) -> DResult<()> {
        Err(Self::new(ctx, DErrorType::GrubParse(message.into())))
    }
}

pub type DResult<T> = core::result::Result<T, DError>;

pub trait DRes<T> {
    fn ctx<M: Into<String>>(self, ctx: DCtx, msg: M) -> DResult<T>;
}

impl<T> DRes<T> for std::io::Result<T> {
    fn ctx<M: Into<String>>(self, ctx: DCtx, msg: M) -> DResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(DError {
                ctx,
                error: DErrorType::Io(msg.into(), err),
            }),
        }
    }
}

impl<T> DRes<T> for sqlx::Result<T> {
    fn ctx<M: Into<String>>(self, ctx: DCtx, msg: M) -> DResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(DError {
                ctx,
                error: DErrorType::Sqlx(msg.into(), err),
            }),
        }
    }
}

impl<T> DRes<T> for zbus::Result<T> {
    fn ctx<M: Into<String>>(self, ctx: DCtx, msg: M) -> DResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(DError {
                ctx,
                error: DErrorType::Zbus(msg.into(), err),
            }),
        }
    }
}
