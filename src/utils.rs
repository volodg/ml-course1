pub trait SomeExt {
    fn some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> SomeExt for T {}

pub trait OkExt {
    fn ok<Err>(self) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(self)
    }
}

impl<T> OkExt for T {}
