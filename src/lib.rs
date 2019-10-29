pub mod prelude {
    pub use anyhow::Result;
    pub use anyhow::Error;
    pub use super::{exec, ok};
}

#[inline(always)]
pub fn ok<E>() -> Result<(), E> {
    Ok(())
}

#[inline(always)]
pub fn exec<T, ErrSrc, ErrDest: From<ErrSrc>>(res: Result<T, ErrSrc>) -> Result<T, ErrDest> {
    let v = res?;
    Ok(v)
}